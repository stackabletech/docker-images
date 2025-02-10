mod error;
mod patch;
mod patch_mail;
mod repo;
mod utils;

use core::str;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use git2::Repository;
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt as _, Snafu};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

#[derive(clap::Parser)]
struct ProductVersion {
    /// The product name slug (such as druid)
    product: String,

    /// The product version (such as 28.0.0)
    ///
    /// Should not contain a v prefix.
    version: String,
}

#[derive(Deserialize, Serialize)]
struct ProductVersionConfig {
    upstream: String,
    base: String,
}

struct ProductVersionContext<'a> {
    pv: ProductVersion,
    images_repo_root: &'a Path,
}

impl ProductVersionContext<'_> {
    fn load_config(&self) -> Result<ProductVersionConfig> {
        let path = &self.config_path();
        tracing::info!(
            config.path = ?path,
            "loading config"
        );
        toml::from_str::<ProductVersionConfig>(
            &std::fs::read_to_string(path).context(LoadConfigSnafu { path })?,
        )
        .context(ParseConfigSnafu { path })
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

/// Patchable is a tool for managing patches for the third-party products distributed by Stackable.
#[derive(clap::Parser)]
#[clap(
    // Encourage people to let cargo decide the current version
    bin_name = "cargo patchable",
)]
struct Opts {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(clap::Parser)]
enum Cmd {
    /// Check out a patched source tree to docker-images/<PRODUCT>/patchable-work/worktree/<VERSION>
    ///
    /// The patches will be pulled from docker-images/<PRODUCT>/stackable/patches/<VERSION>
    ///
    /// The source tree will be overwritten if it already exists (equivalent to `git switch`).
    Checkout {
        #[clap(flatten)]
        pv: ProductVersion,
    },

    /// Export the patches from the source tree at docker-images/<PRODUCT>/patchable-work/worktree/<VERSION>
    ///
    /// The patches will be saved to docker-images/<PRODUCT>/stackable/patches/<VERSION>
    Export {
        #[clap(flatten)]
        pv: ProductVersion,
    },

    /// Creates a patchable.toml for a given product version
    Init {
        #[clap(flatten)]
        pv: ProductVersion,

        /// The upstream URL (such as https://github.com/apache/druid.git)
        #[clap(long)]
        upstream: String,

        /// The upstream commit-ish (such as druid-28.0.0) that the patch series applies to
        ///
        /// Refs (such as tags and branches) will be resolved to commit IDs.
        #[clap(long)]
        base: String,
    },
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to configure git logging"))]
    ConfigureGitLogging { source: git2::Error },

    #[snafu(display("failed to load config from {path:?}"))]
    LoadConfig {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("failed to parse config from {path:?}"))]
    ParseConfig {
        source: toml::de::Error,
        path: PathBuf,
    },
    #[snafu(display("failed to serialize config"))]
    SerializeConfig { source: toml::ser::Error },
    #[snafu(display("failed to create patch dir at {path:?}"))]
    CreatePatchDir {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("failed to save config to {path:?}"))]
    SaveConfig {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to find images repository"))]
    FindImagesRepo { source: git2::Error },
    #[snafu(display("images repository has no work directory"))]
    NoImagesRepoWorkdir,

    #[snafu(display("failed to fetch patch series' base commit"))]
    FetchBaseCommit { source: repo::Error },
    #[snafu(display("failed to apply patch series"))]
    ApplyPatches { source: patch::Error },

    #[snafu(display("failed to open product repository"))]
    OpenProductRepoForCheckout { source: repo::Error },
    #[snafu(display("failed to checkout product worktree"))]
    CheckoutProductWorktree { source: repo::Error },

    #[snafu(display("failed to open product repository at {path:?}"))]
    OpenProductRepo { source: git2::Error, path: PathBuf },
    #[snafu(display("failed to find base commit {commit:?} in repository {repo}"))]
    FindBaseCommit {
        source: git2::Error,
        repo: error::RepoPath,
        commit: String,
    },
    #[snafu(display("failed to find head commit in repository {repo}"))]
    FindHeadCommit {
        source: git2::Error,
        repo: error::RepoPath,
    },
    #[snafu(display("failed to canonicalize history between {base}..{leaf}"))]
    CanonicalizeHistory {
        source: patch::Error,
        base: error::CommitId,
        leaf: error::CommitId,
    },
    #[snafu(display(
        "failed to format patches between {base}..{leaf} (canonicalized from {original_leaf})"
    ))]
    FormatPatches {
        source: patch::Error,
        base: error::CommitId,
        leaf: error::CommitId,
        original_leaf: error::CommitId,
    },
}
type Result<T, E = Error> = std::result::Result<T, E>;

#[snafu::report]
fn main() -> Result<()> {
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
    .context(ConfigureGitLoggingSnafu)?;

    let opts = <Opts as clap::Parser>::parse();
    let images_repo = Repository::discover(".").context(FindImagesRepoSnafu)?;
    let images_repo_root = images_repo.workdir().context(NoImagesRepoWorkdirSnafu)?;
    match opts.cmd {
        Cmd::Checkout { pv } => {
            let ctx = ProductVersionContext {
                pv,
                images_repo_root,
            };
            let config = ctx.load_config()?;
            let product_repo_root = ctx.repo();
            let product_repo = tracing::info_span!(
                "finding product repository",
                product.repository = ?product_repo_root,
            )
            .in_scope(|| repo::ensure_bare_repo(&product_repo_root))
            .context(OpenProductRepoForCheckoutSnafu)?;

            let base_commit =
                repo::resolve_commitish_or_fetch(&product_repo, &config.base, &config.upstream)
                    .context(FetchBaseCommitSnafu)?;
            let patched_commit = patch::apply_patches(&product_repo, &ctx.patch_dir(), base_commit)
                .context(ApplyPatchesSnafu)?;

            let product_worktree_root = ctx.worktree_root();
            repo::ensure_worktree_is_at(
                &product_repo,
                &ctx.pv.version,
                &product_worktree_root,
                &ctx.worktree_branch(),
                patched_commit,
            )
            .context(CheckoutProductWorktreeSnafu)?;

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
            let config = ctx.load_config()?;

            let product_worktree_root = ctx.worktree_root();
            tracing::info!(
                worktree.root = ?product_worktree_root,
                "opening product worktree"
            );
            let product_version_repo =
                Repository::open(&product_worktree_root).context(OpenProductRepoSnafu {
                    path: product_worktree_root,
                })?;

            let base_commit = product_version_repo
                .revparse_single(&config.base)
                .and_then(|c| c.peel_to_commit())
                .context(FindBaseCommitSnafu {
                    repo: &product_version_repo,
                    commit: config.base,
                })?
                .id();
            let original_leaf_commit = product_version_repo
                .head()
                .and_then(|c| c.peel_to_commit())
                .context(FindHeadCommitSnafu {
                    repo: &product_version_repo,
                })?
                .id();
            let canonical_leaf_commit = patch::canonicalize_commit_history(
                &product_version_repo,
                base_commit,
                original_leaf_commit,
            )
            .context(CanonicalizeHistorySnafu {
                base: base_commit,
                leaf: original_leaf_commit,
            })?;

            let patch_dir = ctx.patch_dir();
            patch::format_patches(
                &product_version_repo,
                &patch_dir,
                base_commit,
                canonical_leaf_commit,
            )
            .context(FormatPatchesSnafu {
                base: base_commit,
                leaf: canonical_leaf_commit,
                original_leaf: original_leaf_commit,
            })?;

            tracing::info!(
                patch.dir = ?patch_dir,
                "worktree is exported!"
            );
        }

        Cmd::Init { pv, upstream, base } => {
            let ctx = ProductVersionContext {
                pv,
                images_repo_root,
            };

            let product_repo_root = ctx.repo();
            let product_repo = tracing::info_span!(
                "finding product repository",
                product.repository = ?product_repo_root,
            )
            .in_scope(|| repo::ensure_bare_repo(&product_repo_root))
            .context(OpenProductRepoForCheckoutSnafu)?;

            tracing::info!(?base, "resolving base commit-ish");
            let base_commit = repo::resolve_commitish_or_fetch(&product_repo, &base, &upstream)
                .context(FetchBaseCommitSnafu)?;
            tracing::info!(?base, base.commit = ?base_commit, "resolved base commit");

            let config = ProductVersionConfig {
                upstream,
                base: base.to_string(),
            };
            let config_path = ctx.config_path();
            if let Some(config_dir) = config_path.parent() {
                std::fs::create_dir_all(config_dir)
                    .context(CreatePatchDirSnafu { path: config_dir })?;
            }
            let config_toml = toml::to_string_pretty(&config).context(SerializeConfigSnafu)?;
            File::create_new(&config_path)
                .and_then(|mut f| f.write_all(config_toml.as_bytes()))
                .context(SaveConfigSnafu { path: config_path })?;
        }
    }

    Ok(())
}
