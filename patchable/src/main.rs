use std::path::{Path, PathBuf};

use git2::{
    build::RepoBuilder, ObjectType, Repository, Signature, StatusOptions, WorktreeAddOptions,
};
use regex::{Regex, RegexBuilder};
use serde::Deserialize;

#[derive(clap::Parser)]
struct ProductVersion {
    product: String,
    version: String,
}

impl ProductVersion {
    fn root(&self, images_root: &Path) -> PathBuf {
        images_root.join(&self.product)
    }

    fn patch_dir(&self, images_root: &Path) -> PathBuf {
        self.root(images_root)
            .join("stackable/patches")
            .join(&self.version)
    }

    fn config_path(&self, images_root: &Path) -> PathBuf {
        self.patch_dir(images_root).join("patchable.toml")
    }

    fn work_root(&self, images_root: &Path) -> PathBuf {
        self.root(images_root).join("patchable-work")
    }

    fn repo(&self, images_root: &Path) -> PathBuf {
        self.work_root(images_root).join("product-repo")
    }

    fn worktree_root(&self, images_root: &Path) -> PathBuf {
        self.work_root(images_root)
            .join("worktree")
            .join(&self.version)
    }

    fn worktree_branch(&self) -> String {
        format!("patchable/{}", self.version)
    }
}

#[derive(Deserialize)]
struct ProductVersionConfig {
    upstream: String,
    base: String,
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
            let config = toml::from_str::<ProductVersionConfig>(
                &std::fs::read_to_string(pv.config_path(images_repo_root)).unwrap(),
            )
            .unwrap();
            let product_repo_root = pv.repo(images_repo_root);
            let product_repo = match Repository::open(&product_repo_root) {
                Ok(repo) => repo,
                Err(_) => RepoBuilder::new()
                    .bare(true)
                    .clone(&config.upstream, &product_repo_root)
                    .unwrap(),
            };
            let product_worktree_root = pv.worktree_root(images_repo_root);
            let mut product_version_repo = match Repository::open(&product_worktree_root) {
                Ok(wt) => wt,
                Err(err) => {
                    tracing::info!(
                        error = &err as &dyn std::error::Error,
                        "worktree not found, creating"
                    );
                    Repository::open_from_worktree(
                        &product_repo
                            .worktree(
                                &pv.version,
                                &product_worktree_root,
                                Some(&WorktreeAddOptions::new()),
                            )
                            .unwrap(),
                    )
                    .unwrap()
                }
            };
            product_version_repo.cleanup_state().unwrap();

            let stash = if product_version_repo
                .statuses(Some(StatusOptions::new().include_unmodified(false)))
                .unwrap()
                .is_empty()
            {
                None
            } else {
                Some(
                    product_version_repo
                        .stash_save(
                            &Signature::now("Patchable", "noreply+patchable@stackable.tech")
                                .unwrap(),
                            "Existing work before checking out ",
                            None,
                        )
                        .unwrap(),
                )
            };

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
                        &pv.worktree_branch(),
                        &product_version_repo
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

            let patch_dir = pv.patch_dir(images_repo_root);
            let series_file = patch_dir.join("series");
            let mut apply_cmd = raw_git_cmd(&product_version_repo);
            if series_file.exists() {
                apply_cmd
                    .arg("am")
                    .arg(series_file)
                    .args(["--patch-format", "stgit-series"]);
            } else {
                let mut patch_files = patch_dir
                    .read_dir()
                    .unwrap()
                    .map(|x| x.unwrap().path())
                    .filter(|x| x.extension().is_some_and(|x| x == "patch"))
                    .collect::<Vec<_>>();
                patch_files.sort();
                apply_cmd.arg("am").args(patch_files);
            }
            if !apply_cmd.spawn().unwrap().wait().unwrap().success() {
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
                product_version_repo
                    .stash_pop(stash_index.unwrap(), None)
                    .unwrap();
            }
        }
        Cmd::Export { pv } => {
            let config = toml::from_str::<ProductVersionConfig>(
                &std::fs::read_to_string(pv.config_path(images_repo_root)).unwrap(),
            )
            .unwrap();

            let patch_dir = pv.patch_dir(images_repo_root);
            for entry in patch_dir.read_dir().unwrap() {
                let path = entry.unwrap().path();
                if path.file_name().is_some_and(|x| x == "series")
                    || path.extension().is_some_and(|x| x == "patch")
                {
                    std::fs::remove_file(path).unwrap();
                }
            }

            let product_version_repo =
                Repository::open(pv.worktree_root(images_repo_root)).unwrap();
            if !raw_git_cmd(&product_version_repo)
                .arg("format-patch")
                .arg(&config.base)
                .arg("-o")
                .arg(&patch_dir)
                .arg("--base")
                .arg(&config.base)
                .arg("--keep-subject")
                .spawn()
                .unwrap()
                .wait()
                .unwrap()
                .success()
            {
                panic!("failed to format patches");
            }

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
