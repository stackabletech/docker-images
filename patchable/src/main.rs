use core::str;
use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Stdio,
};

use git2::{
    Diff, FetchOptions, ObjectType, Repository, RepositoryInitOptions, Signature,
    WorktreeAddOptions,
};
use serde::Deserialize;
use tempfile::tempdir;
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

#[derive(clap::Parser)]
struct ProductVersion {
    product: String,
    version: String,
}

#[derive(Deserialize)]
struct ProductVersionConfig {
    upstream: String,
    base: String,
}

struct ProductVersionContext<'a> {
    pv: ProductVersion,
    images_repo_root: &'a Path,
}

impl ProductVersionContext<'_> {
    fn load_config(&self) -> ProductVersionConfig {
        tracing::info!(
            config.path = %self.config_path().display(),
            "loading config"
        );
        toml::from_str::<ProductVersionConfig>(
            &std::fs::read_to_string(self.config_path()).unwrap(),
        )
        .unwrap()
    }

    fn root(&self) -> PathBuf {
        self.images_repo_root.join(&self.pv.product)
    }

    fn patch_dir(&self) -> PathBuf {
        self.root().join("stackable/patches").join(&self.pv.version)
    }

    fn config_path(&self) -> PathBuf {
        self.patch_dir().join("patchable.toml")
    }

    fn work_root(&self) -> PathBuf {
        self.root().join("patchable-work")
    }

    fn repo(&self) -> PathBuf {
        self.work_root().join("product-repo")
    }

    fn worktree_root(&self) -> PathBuf {
        self.work_root().join("worktree").join(&self.pv.version)
    }

    fn worktree_branch(&self) -> String {
        format!("patchable/{}", self.pv.version)
    }
}

#[derive(clap::Parser)]
struct Opts {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(clap::Parser)]
enum Cmd {
    Checkout {
        #[clap(flatten)]
        pv: ProductVersion,
    },
    Export {
        #[clap(flatten)]
        pv: ProductVersion,
    },
}

fn main() {
    tracing_subscriber::fmt().init();
    let opts = <Opts as clap::Parser>::parse();
    let images_repo = Repository::discover(".").unwrap();
    let images_repo_root = images_repo.workdir().unwrap();
    match opts.cmd {
        Cmd::Checkout { pv } => {
            let ctx = ProductVersionContext {
                pv,
                images_repo_root,
            };
            let config = ctx.load_config();
            let product_repo_root = ctx.repo();
            let product_repo = match Repository::open(&product_repo_root) {
                Ok(repo) => {
                    tracing::info!(
                        product.repository = %product_repo_root.display(),
                        "product repository found, reusing"
                    );
                    repo
                }
                Err(err) => {
                    tracing::info!(
                        error = &err as &dyn std::error::Error,
                        product.repository = %product_repo_root.display(),
                        "product repository not found, initializing"
                    );
                    let repo = Repository::init_opts(
                        &product_repo_root,
                        RepositoryInitOptions::new()
                            .bare(true)
                            .initial_head("patchable-dummy")
                            .external_template(false),
                    )
                    .unwrap();

                    repo
                }
            };

            match product_repo.revparse_single(&config.base) {
                Ok(_) => tracing::info!(
                    worktree.branch.base = config.base,
                    "base commit exists, reusing"
                ),
                Err(err) => {
                    tracing::info!(
                        error = &err as &dyn std::error::Error,
                        worktree.branch.base = config.base,
                        product.upstream = config.upstream,
                        "base commit not found, fetching from upstream"
                    );
                    product_repo
                        .remote_anonymous(&config.upstream)
                        .unwrap()
                        .fetch(
                            &[&config.base],
                            Some(
                                FetchOptions::new()
                                    // TODO: could be 1, CLI option maybe?
                                    .depth(0),
                            ),
                            None,
                        )
                        .unwrap();
                }
            }

            let product_worktree_root = ctx.worktree_root();
            let worktree_branch = ctx.worktree_branch();
            let product_version_repo = match Repository::open(&product_worktree_root) {
                Ok(wt) => {
                    tracing::info!(
                        worktree.root = %product_worktree_root.display(),
                        "worktree found, resetting and reusing"
                    );
                    wt
                }
                Err(err) => {
                    tracing::info!(
                        error = &err as &dyn std::error::Error,
                        worktree.root = %product_worktree_root.display(),
                        product.repository = %product_repo_root.display(),
                        "worktree not found, creating"
                    );
                    std::fs::create_dir_all(product_worktree_root.parent().unwrap()).unwrap();
                    let worktree_ref = product_repo
                        .branch(
                            &worktree_branch,
                            &product_repo
                                .revparse_single(&config.base)
                                .unwrap()
                                .peel_to_commit()
                                .unwrap(),
                            true,
                        )
                        .unwrap()
                        .into_reference();
                    Repository::open_from_worktree(
                        &product_repo
                            .worktree(
                                &ctx.pv.version,
                                &product_worktree_root,
                                Some(WorktreeAddOptions::new().reference(Some(&worktree_ref))),
                            )
                            .unwrap(),
                    )
                    .unwrap()
                }
            };

            tracing::info!("cleaning up existing rebase state");
            product_version_repo.cleanup_state().unwrap();

            let patch_dir = ctx.patch_dir();
            tracing::info!(
                patch.dir = %patch_dir.display(),
                "applying patches"
            );
            // This effectively reimplements git-am, but lets us:
            // 1. Fake the committer information to match author (to ensure that we get deterministic committer IDs across machines)
            // 2. Avoid touching the worktree until all patches have been applied
            //   (letting us keep any dirty files in the worktree that don't conflict with the final switcheroo,
            //   even if those files are modified by some of the patches)
            let series_file = patch_dir.join("series");
            let patch_files = if series_file.exists() {
                tracing::info!(
                    patch.series = %series_file.display(),
                    "series file found, treating as stgit series"
                );
                std::fs::read_to_string(series_file)
                    .unwrap()
                    .lines()
                    .map(|file_name| patch_dir.join(file_name))
                    .collect::<Vec<_>>()
            } else {
                tracing::info!(
                    patch.series = %series_file.display(),
                    "series file not found, treating as git mailbox"
                );
                let mut patch_files = patch_dir
                    .read_dir()
                    .unwrap()
                    .map(|x| x.unwrap().path())
                    .filter(|x| x.extension().is_some_and(|x| x == "patch"))
                    .collect::<Vec<_>>();
                patch_files.sort();
                patch_files
            };
            let mut last_commit_id = product_version_repo
                .revparse_single(&config.base)
                .unwrap()
                .id();
            for patch_file in patch_files {
                tracing::info!(
                    patch.file = %patch_file.display(),
                    "parsing patch"
                );
                let mailsplit_dir = tempdir().unwrap();
                let mailsplit = raw_git_cmd(&product_version_repo)
                    .arg("mailsplit")
                    // mailsplit doesn't accept split arguments ("-o dir")
                    .arg(format!("-o{}", mailsplit_dir.path().to_str().unwrap()))
                    .arg("--")
                    .arg(patch_file)
                    .stderr(Stdio::inherit())
                    .output()
                    .unwrap();
                if !mailsplit.status.success() {
                    panic!("failed to apply patches");
                }
                let mailsplit_patch_count = str::from_utf8(&mailsplit.stdout)
                    .unwrap()
                    .trim()
                    .parse::<u32>()
                    .unwrap();
                for patch_i in 1..=mailsplit_patch_count {
                    // Matches the format emitted by git-mailsplit
                    let patch_mail_file_name = format!("{patch_i:04}");
                    let patch_split_msg_file = mailsplit_dir
                        .path()
                        .join(format!("{patch_mail_file_name}-msg"));
                    let patch_split_patch_file = mailsplit_dir
                        .path()
                        .join(format!("{patch_mail_file_name}-patch"));
                    let mailinfo = raw_git_cmd(&product_version_repo)
                        .arg("mailinfo")
                        .args([&patch_split_msg_file, &patch_split_patch_file])
                        .stdin(File::open(mailsplit_dir.path().join(patch_mail_file_name)).unwrap())
                        .stderr(Stdio::inherit())
                        .output()
                        .unwrap();
                    if !mailinfo.status.success() {
                        panic!("failed to apply patches");
                    }
                    let patch_info = str::from_utf8(&mailinfo.stdout).unwrap();

                    let mut author_name = None;
                    let mut author_email = None;
                    let mut date = None;
                    let mut subject = None;
                    for patch_info_line in patch_info.lines() {
                        if !patch_info_line.is_empty() {
                            match patch_info_line.split_once(": ").unwrap() {
                                ("Author", x) => author_name = Some(x),
                                ("Email", x) => author_email = Some(x),
                                ("Date", x) => date = Some(x),
                                ("Subject", x) => subject = Some(x),
                                (header, _) => panic!("unknown header type {header:?}"),
                            }
                        }
                    }
                    let date = OffsetDateTime::parse(date.unwrap(), &Rfc2822).unwrap();
                    let author = Signature::new(
                        author_name.unwrap(),
                        author_email.unwrap(),
                        &git2::Time::new(
                            date.unix_timestamp(),
                            date.offset().whole_minutes().into(),
                        ),
                    )
                    .unwrap();
                    let parent_commit = product_version_repo.find_commit(last_commit_id).unwrap();
                    tracing::info!(
                        commit.base = %parent_commit.id(),
                        commit.subject = subject.unwrap(),
                        "applying patch"
                    );
                    let patch_tree_id = product_version_repo
                        .apply_to_tree(
                            &parent_commit.tree().unwrap(),
                            &Diff::from_buffer(&std::fs::read(patch_split_patch_file).unwrap())
                                .unwrap(),
                            None,
                        )
                        .unwrap()
                        .write_tree_to(&product_version_repo)
                        .unwrap();
                    let msg = std::fs::read_to_string(patch_split_msg_file).unwrap();
                    let full_msg = if msg.is_empty() {
                        subject.unwrap()
                    } else {
                        &format!("{}\n\n{msg}", subject.unwrap())
                    };
                    last_commit_id = product_version_repo
                        .commit(
                            None,
                            &author,
                            &author,
                            full_msg,
                            &product_version_repo.find_tree(patch_tree_id).unwrap(),
                            &[&parent_commit],
                        )
                        .unwrap();
                    tracing::info!(
                        commit.id = %last_commit_id,
                        "applied patch"
                    );
                }
            }

            tracing::info!(
                worktree.branch = worktree_branch,
                worktree.branch.commit = %last_commit_id,
                worktree.branch.base = config.base,
                "checking out branch"
            );
            // We can't reset the branch if it's already checked out, so detach to the commit instead for the meantime
            product_version_repo
                .set_head_detached(
                    product_version_repo
                        .head()
                        .unwrap()
                        .peel_to_commit()
                        .unwrap()
                        .id(),
                )
                .unwrap();
            {
                let branch = product_version_repo
                    .branch(
                        &worktree_branch,
                        &product_version_repo.find_commit(last_commit_id).unwrap(),
                        true,
                    )
                    .unwrap()
                    .into_reference();
                product_version_repo
                    .checkout_tree(&branch.peel(ObjectType::Commit).unwrap(), None)
                    .unwrap();
                product_version_repo
                    .set_head_bytes(branch.name_bytes())
                    .unwrap();
            }

            tracing::info!(
                worktree.root = %product_worktree_root.display(),
                "worktree is ready!"
            );
        }
        Cmd::Export { pv } => {
            let ctx = ProductVersionContext {
                pv,
                images_repo_root,
            };
            let config = ctx.load_config();

            let product_worktree_root = ctx.worktree_root();
            tracing::info!(
                worktree.root = %product_worktree_root.display(),
                "opening product worktree"
            );
            let product_version_repo = Repository::open(&product_worktree_root).unwrap();

            let patch_dir = ctx.patch_dir();
            tracing::info!(
                patch.dir = %patch_dir.display(),
                "deleting existing patch files"
            );
            // git format-patch is happy to overwrite existing files,
            // but we also want to delete removed (or renamed) patch files.
            for entry in patch_dir.read_dir().unwrap() {
                let path = entry.unwrap().path();
                // git format-patch emits the mailbox format, not stgit(/quilt),
                // so also remove markers that make it look like that
                if path.file_name().is_some_and(|x| x == "series")
                    || path.extension().is_some_and(|x| x == "patch")
                {
                    std::fs::remove_file(path).unwrap();
                }
            }

            tracing::info!(
                patch.dir = %patch_dir.display(),
                worktree.root = %product_worktree_root.display(),
                worktree.base = config.base,
                "exporting commits since base"
            );
            if !raw_git_cmd(&product_version_repo)
                .arg("format-patch")
                .arg(&config.base)
                .arg("-o")
                .arg(&patch_dir)
                .arg("--base")
                .arg(&config.base)
                .arg("--keep-subject")
                .status()
                .unwrap()
                .success()
            {
                panic!("failed to format patches");
            }

            tracing::info!(
                patch.dir = %patch_dir.display(),
                "worktree is exported!"
            );
        }
    }
}

/// Runs a raw git command in the environment of a Git repository.
///
/// Used for functionality that is not currently implemented by libgit2/gix.
fn raw_git_cmd(repo: &Repository) -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    cmd.env("GIT_DIR", repo.path())
        .env("GIT_WORK_TREE", repo.workdir().unwrap());
    cmd
}
