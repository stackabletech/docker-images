use std::path::{Path, PathBuf};

use git2::{
    FetchOptions, ObjectType, Repository, RepositoryInitOptions, Signature, StatusOptions,
    WorktreeAddOptions,
};
use regex::{Regex, RegexBuilder};
use serde::Deserialize;

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
                        product.upstream = config.upstream,
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
            let mut product_version_repo = match Repository::open(&product_worktree_root) {
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

            let stash = if product_version_repo
                .statuses(Some(StatusOptions::new().include_unmodified(false)))
                .unwrap()
                .is_empty()
            {
                tracing::info!("worktree is clean, no need to stash");
                None
            } else {
                tracing::warn!("worktree is dirty, stashing changes!");
                let stash = product_version_repo
                    .stash_save(
                        &Signature::now("Patchable", "noreply+patchable@stackable.tech").unwrap(),
                        "Existing work before checking out ",
                        None,
                    )
                    .unwrap();
                tracing::info!(%stash, "created stash");
                Some(stash)
            };

            tracing::info!(
                worktree.branch = worktree_branch,
                worktree.branch.base = config.base,
                "checking out base commit into branch"
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
                        product_version_repo
                            .revparse_single(&config.base)
                            .unwrap()
                            .as_commit()
                            .unwrap(),
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

            let patch_dir = ctx.patch_dir();
            tracing::info!(
                patch.dir = %patch_dir.display(),
                "applying patches"
            );
            let series_file = patch_dir.join("series");
            let mut apply_cmd = raw_git_cmd(&product_version_repo);
            if series_file.exists() {
                tracing::info!(
                    patch.series = %series_file.display(),
                    "series file found, treating as stgit series"
                );
                apply_cmd
                    .arg("am")
                    .arg(series_file)
                    .args(["--patch-format", "stgit-series"]);
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
                apply_cmd.arg("am").args(patch_files);
            }
            if !apply_cmd.status().unwrap().success() {
                panic!("failed to apply patches");
            }

            if let Some(stash) = stash {
                let mut stash_index = None;
                product_version_repo
                    .stash_foreach(|i, _, oid| {
                        if oid == &stash {
                            stash_index = Some(i);
                            true
                        } else {
                            false
                        }
                    })
                    .unwrap();
                let stash_index = stash_index.unwrap();
                tracing::info!(
                    stash = %format_args!("stash@{{{stash_index}}}"),
                    "restoring stash"
                );
                product_version_repo.stash_pop(stash_index, None).unwrap();
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

            // Normally the patches include their own commit IDs, which will change for every for every re-checkout
            // checkout doesn't actually care about this value, so we can just replace it with a deterministic dummy
            tracing::info!("scrubbing commit ID from exported patches");
            let regex_line = RegexBuilder::new("^.*$").multi_line(true).build().unwrap();
            let regex_from = Regex::new("^From [0-9a-f]+ ").unwrap();
            for entry in patch_dir.read_dir().unwrap() {
                let path = entry.unwrap().path();
                if path.extension().is_some_and(|x| x == "patch") {
                    let mut patch_file = std::fs::read_to_string(&path).unwrap();
                    let line_1 = regex_line.find(&patch_file).unwrap();
                    let from = regex_from
                        .find_at(&patch_file[..line_1.end()], line_1.start())
                        .unwrap();
                    patch_file.replace_range(
                        from.range(),
                        "From 0000000000000000000000000000000000000000 ",
                    );
                    std::fs::write(path, patch_file).unwrap();
                }
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
