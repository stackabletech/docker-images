mod patch;
mod repo;
mod utils;

use core::str;
use std::path::{Path, PathBuf};

use git2::Repository;
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

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
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
    git2::trace_set(git2::TraceLevel::Trace, |level, msg| {
        let msg = String::from_utf8_lossy(msg);
        match level {
            git2::TraceLevel::None | git2::TraceLevel::Fatal | git2::TraceLevel::Error => {
                tracing::error!(target: "git", "{msg}")
            }
            git2::TraceLevel::Warn => tracing::warn!(target: "git", "{msg}"),
            git2::TraceLevel::Info => tracing::info!(target: "git", "{msg}"),
            git2::TraceLevel::Debug => tracing::debug!(target: "git", "{msg}"),
            git2::TraceLevel::Trace => tracing::trace!(target: "git", "{msg}"),
        }
    })
    .unwrap();

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
            let product_repo = tracing::info_span!(
                "finding product repository",
                product.repository = ?product_repo_root,
            )
            .in_scope(|| repo::ensure_bare_repo(&product_repo_root));

            let base_commit =
                repo::ensure_commit_exists_or_pull(&product_repo, &config.base, &config.upstream);
            let patched_commit = patch::apply_patches(&product_repo, &ctx.patch_dir(), base_commit);

            let product_worktree_root = ctx.worktree_root();
            repo::ensure_worktree_is_at(
                &product_repo,
                &ctx.pv.version,
                &product_worktree_root,
                &ctx.worktree_branch(),
                patched_commit,
            );

            tracing::info!(
                worktree.root = ?product_worktree_root,
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
                worktree.root = ?product_worktree_root,
                "opening product worktree"
            );
            let product_version_repo = Repository::open(&product_worktree_root).unwrap();

            let base_commit = product_version_repo
                .revparse_single(&config.base)
                .unwrap()
                .peel_to_commit()
                .unwrap()
                .id();
            let canonical_leaf_commit = patch::canonicalize_commit_history(
                &product_version_repo,
                base_commit,
                product_version_repo
                    .head()
                    .unwrap()
                    .peel_to_commit()
                    .unwrap()
                    .id(),
            );

            let patch_dir = ctx.patch_dir();
            patch::format_patches(
                &product_version_repo,
                &patch_dir,
                base_commit,
                canonical_leaf_commit,
            );

            tracing::info!(
                patch.dir = ?patch_dir,
                "worktree is exported!"
            );
        }
    }
}
