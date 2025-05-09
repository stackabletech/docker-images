mod error;
mod patch;
mod patch_mail;
mod repo;
mod utils;

use core::str;
use std::{fs::File, io::Write, path::PathBuf};

use git2::{Oid, Repository};
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt as _, Snafu};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

use crate::utils::setup_git_credentials;

#[derive(clap::Parser)]
struct ProductVersion {
    /// The product name slug (such as druid)
    product: String,

    /// The product version (such as 28.0.0)
    ///
    /// Should not contain a v prefix.
    version: String,
}

/// Configuration that applies to all versions of a product, located at $ROOT/$product/stackable/patches/patchable.toml
///
/// Must be created by hand (for now).
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct ProductConfig {
    /// The upstream repository URL
    upstream: String,

    /// The repository that commits are mirrored to by `init --mirror`, typically `https://github.com/stackabletech/$product.git`
    ///
    /// This value is _not_ used by `checkout`, that uses [`ProductVersionConfig::mirror`] instead.
    /// `init --mirror` copies this value into [`ProductVersionConfig::mirror`].
    #[serde(skip_serializing_if = "Option::is_none")]
    default_mirror: Option<String>,
}

/// Configuration that applies to an individual version of a product, located at $ROOT/$product/stackable/patches/$version/patchable.toml
///
/// Typically created by `patchable init`.
#[derive(Deserialize, Serialize)]
struct ProductVersionConfig {
    /// The mirror repository that this version can be fetched from
    ///
    /// Copied from [`ProductConfig::default_mirror`] by `init --mirror`.
    mirror: Option<String>,

    /// The upstream base commit for this version
    ///
    /// Must be a commit ID, not a generic commitish (branch, tag, or anotherother ref), those are resolved by `init`.
    #[serde(with = "utils::oid_serde")]
    base: Oid,
}

struct ProductVersionContext {
    pv: ProductVersion,
    images_repo_root: PathBuf,
}

impl ProductVersionContext {
    fn load_config<T: for<'de> Deserialize<'de>>(&self, path: &PathBuf) -> Result<T> {
        tracing::info!(
            config.path = ?path,
            "loading config"
        );

        toml::from_str::<T>(&std::fs::read_to_string(path).context(LoadConfigSnafu { path })?)
            .context(ParseConfigSnafu { path })
    }

    fn load_product_config(&self) -> Result<ProductConfig> {
        let path = self.product_config_path();
        self.load_config(&path)
    }

    fn load_version_config(&self) -> Result<ProductVersionConfig> {
        let path = self.version_config_path();
        self.load_config(&path)
    }

    /// The root directory for files related to the product (across all versions).
    fn product_dir(&self) -> PathBuf {
        self.images_repo_root.join(&self.pv.product)
    }

    /// The directory containing patches for the product version.
    fn patch_dir(&self) -> PathBuf {
        self.product_dir()
            .join("stackable/patches")
            .join(&self.pv.version)
    }

    /// The patchable configuration file for the product version.
    fn version_config_path(&self) -> PathBuf {
        self.patch_dir().join("patchable.toml")
    }

    /// The product-level patchable configuration file
    fn product_config_path(&self) -> PathBuf {
        self.product_dir().join("stackable/patches/patchable.toml")
    }

    /// The directory containing all ephemeral data used by patchable for the product (across all versions).
    ///
    /// Should be gitignored, and can safely be deleted as long as all relevant versions have been `patchable export`ed.
    fn work_root(&self) -> PathBuf {
        self.product_dir().join("patchable-work")
    }

    /// The repository for the product (across all versions).
    fn product_repo(&self) -> PathBuf {
        self.work_root().join("product-repo")
    }

    /// The worktree root for the product version.
    fn worktree_root(&self) -> PathBuf {
        self.work_root().join("worktree").join(&self.pv.version)
    }

    /// Branch pointing at the upstream base commit for the product version.
    fn base_branch(&self) -> String {
        format!("patchable/base/{}", self.pv.version)
    }

    /// branch pointing at the last commit in the patch series for the product version.
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

    /// Specify a custom root directory for the images repository
    #[clap(long)]
    images_repo_root: Option<PathBuf>,
}

#[derive(clap::Parser)]
// CLI parameters are documented for the CLI's `--help`, not for rustdoc
#[allow(rustdoc::bare_urls, rustdoc::invalid_html_tags)]
enum Cmd {
    /// Check out a patched source tree to docker-images/<PRODUCT>/patchable-work/worktree/<VERSION>
    ///
    /// The patches will be pulled from docker-images/<PRODUCT>/stackable/patches/<VERSION>
    ///
    /// The source tree will be overwritten if it already exists (equivalent to `git switch`).
    Checkout {
        #[clap(flatten)]
        pv: ProductVersion,

        /// Check out the base commit, without applying patches
        #[clap(long)]
        base_only: bool,

        /// Use SSH for git operations
        #[clap(long)]
        ssh: bool,
    },

    /// Export the patches from the source tree at docker-images/<PRODUCT>/patchable-work/worktree/<VERSION>
    ///
    /// The patches will be saved to docker-images/<PRODUCT>/stackable/patches/<VERSION>
    Export {
        #[clap(flatten)]
        pv: ProductVersion,
    },

    /// Creates patchable.toml configuration files
    Init {
        #[clap(subcommand)]
        init_type: InitType,
    },

    /// Shows the patch directory for a given product version
    PatchDir {
        #[clap(flatten)]
        pv: ProductVersion,
    },

    /// Shows the worktree directory for a given product version
    ///
    /// This is the same value as `cargo patchable checkout` returns, but does not perform a checkout.
    WorktreeDir {
        #[clap(flatten)]
        pv: ProductVersion,
    },

    /// Shows the images repository root
    ImagesDir,
}

#[derive(clap::Parser)]
enum InitType {
    /// Creates a patchable.toml for a given product
    Product {
        /// The product name slug (such as druid)
        product: String,
        /// The upstream repository URL (e.g. https://github.com/apache/druid.git)
        #[clap(long)]
        upstream: String,
        /// The default mirror repository URL (e.g. https://github.com/stackabletech/druid.git)
        #[clap(long)]
        default_mirror: Option<String>,
    },

    /// Creates a patchable.toml for a given product version
    Version {
        #[clap(flatten)]
        pv: ProductVersion,

        /// The upstream commit-ish (such as druid-28.0.0) that the patch series applies to
        ///
        /// Refs (such as tags and branches) will be resolved to commit IDs.
        #[clap(long)]
        base: String,

        /// Mirror the product version to the default mirror repository
        #[clap(long)]
        mirror: bool,

        /// Use SSH for git operations
        #[clap(long)]
        ssh: bool,
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

    #[snafu(display("failed to rewrite URL for SSH"))]
    UrlRewrite { source: utils::UrlRewriteError },

    #[snafu(display(
        "mirroring requested, but default-mirror is not configured in product configuration"
    ))]
    InitMirrorNotConfigured,
    #[snafu(display("failed to add temporary mirror remote for {url:?}"))]
    AddMirrorRemote { source: git2::Error, url: String },
    #[snafu(display("failed to push commit {commit} (as {refspec}) to mirror {url:?}"))]
    PushToMirror {
        source: git2::Error,
        url: String,
        refspec: String,
        commit: Oid,
    },

    #[snafu(display("failed to find images repository"))]
    FindImagesRepo { source: repo::Error },
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
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .with(IndicatifLayer::new())
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
    let images_repo_root = match opts.images_repo_root {
        Some(path) => path,
        None => {
            let images_repo = repo::discover_images_repo(".").context(FindImagesRepoSnafu)?;
            images_repo
                .workdir()
                .context(NoImagesRepoWorkdirSnafu)?
                .to_owned()
        }
    };
    match opts.cmd {
        Cmd::Checkout { pv, base_only, ssh } => {
            let ctx = ProductVersionContext {
                pv,
                images_repo_root,
            };
            let product_config = ctx.load_product_config()?;
            let version_config = ctx.load_version_config()?;
            let product_repo_root = ctx.product_repo();
            let product_repo = repo::ensure_bare_repo(&product_repo_root)
                .context(OpenProductRepoForCheckoutSnafu)?;

            let mut upstream = version_config.mirror.unwrap_or_else(|| {
                    tracing::warn!("this product version is not mirrored, re-init it with --mirror before merging it");
                    product_config.upstream
                });

            if ssh {
                upstream =
                    utils::rewrite_git_https_url_to_ssh(&upstream).context(UrlRewriteSnafu)?;
            }

            let base_commit = repo::resolve_and_fetch_commitish(
                &product_repo,
                &version_config.base.to_string(),
                &upstream,
            )
            .context(FetchBaseCommitSnafu)?;
            let base_branch = ctx.base_branch();
            let base_branch = match product_repo
                .find_commit(base_commit)
                .and_then(|base| product_repo.branch(&base_branch, &base, true))
            {
                Ok(_) => {
                    tracing::info!(
                        branch.base = base_branch,
                        branch.base.commit = %base_commit,
                        "updated base branch"
                    );
                    Some(base_branch)
                }
                Err(err) => {
                    tracing::warn!(
                        error = &err as &dyn std::error::Error,
                        branch.base = base_branch,
                        branch.base.commit = %base_commit,
                        "failed to update base branch reference, ignoring..."
                    );
                    None
                }
            };
            let patched_commit = if !base_only {
                patch::apply_patches(&product_repo, &ctx.patch_dir(), base_commit)
                    .context(ApplyPatchesSnafu)?
            } else {
                tracing::warn!("--base-only specified, skipping patches");
                base_commit
            };

            let product_worktree_root = ctx.worktree_root();
            let worktree_branch = ctx.worktree_branch();
            repo::ensure_worktree_is_at(
                &product_repo,
                &ctx.pv.version,
                &product_worktree_root,
                &worktree_branch,
                patched_commit,
            )
            .context(CheckoutProductWorktreeSnafu)?;

            tracing::info!(
                worktree.root = ?product_worktree_root,
                branch.worktree = worktree_branch,
                branch.base = base_branch,
                "worktree is ready!"
            );

            // Print directory so you can run `cd $(cargo patchable checkout ...)`
            println!("{}", product_worktree_root.display());
        }

        Cmd::Export { pv } => {
            let ctx = ProductVersionContext {
                pv,
                images_repo_root,
            };
            let config = ctx.load_version_config()?;

            let product_worktree_root = ctx.worktree_root();
            tracing::info!(
                worktree.root = ?product_worktree_root,
                "opening product worktree"
            );
            let product_version_repo =
                Repository::open(&product_worktree_root).context(OpenProductRepoSnafu {
                    path: product_worktree_root,
                })?;

            let base_commit = config.base;
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

        Cmd::Init { init_type } => match init_type {
            InitType::Product {
                product,
                upstream,
                default_mirror,
            } => {
                let product_config_path = ProductVersionContext {
                    pv: ProductVersion {
                        product: product.clone(),
                        version: "".to_string(),
                    },
                    images_repo_root,
                }
                .product_config_path();

                tracing::info!(
                    path = ?product_config_path,
                    "creating product configuration directory and file"
                );

                let product_config_dir = product_config_path
                    .parent()
                    .expect("product config should have a hard-coded parent");

                std::fs::create_dir_all(product_config_dir).context(CreatePatchDirSnafu {
                    path: product_config_dir,
                })?;

                let product_config = ProductConfig {
                    upstream,
                    default_mirror,
                };

                let config_toml =
                    toml::to_string_pretty(&product_config).context(SerializeConfigSnafu)?;
                File::create_new(&product_config_path)
                    .and_then(|mut f| f.write_all(config_toml.as_bytes()))
                    .context(SaveConfigSnafu {
                        path: &product_config_path,
                    })?;

                tracing::info!(
                    config.path = ?product_config_path,
                    product = product,
                    "created configuration for product"
                );
            }

            InitType::Version {
                pv,
                base,
                mirror,
                ssh,
            } => {
                let ctx = ProductVersionContext {
                    pv,
                    images_repo_root,
                };

                let product_repo_root = ctx.product_repo();
                let product_repo = tracing::info_span!(
                    "finding product repository",
                    product.repository = ?product_repo_root,
                )
                .in_scope(|| repo::ensure_bare_repo(&product_repo_root))
                .context(OpenProductRepoForCheckoutSnafu)?;

                let config = ctx.load_product_config()?;
                let upstream = if ssh {
                    utils::rewrite_git_https_url_to_ssh(&config.upstream)
                        .context(UrlRewriteSnafu)?
                } else {
                    config.upstream
                };

                // --base can be a reference, but patchable.toml should always have a resolved commit id,
                // so that it cannot be changed under our feet (without us knowing so, anyway...).
                tracing::info!(?base, "resolving base commit-ish");
                let base_commit =
                    repo::resolve_and_fetch_commitish(&product_repo, &base, &upstream)
                        .context(FetchBaseCommitSnafu)?;
                tracing::info!(?base, base.commit = ?base_commit, "resolved base commit");

                let mirror_url = if mirror {
                    let mut mirror_url = config
                        .default_mirror
                        .context(InitMirrorNotConfiguredSnafu)?;
                    if ssh {
                        mirror_url = utils::rewrite_git_https_url_to_ssh(&mirror_url)
                            .context(UrlRewriteSnafu)?
                    };
                    // Add mirror remote
                    let mut mirror_remote = product_repo.remote_anonymous(&mirror_url).context(
                        AddMirrorRemoteSnafu {
                            url: mirror_url.clone(),
                        },
                    )?;

                    // Push the base commit to the mirror
                    tracing::info!(commit = %base_commit, base = base, url = mirror_url, "pushing commit to mirror");
                    let mut callbacks = setup_git_credentials();

                    // Add progress tracking for push operation
                    let (span_push, mut quant_push) =
                        utils::setup_progress_tracking(tracing::info_span!("pushing"));
                    let _ = span_push.enter();

                    callbacks.push_transfer_progress(move |current, total, _| {
                        if total > 0 {
                            quant_push.update_span_progress(current, total, &span_push);
                        }
                    });

                    let mut push_options = git2::PushOptions::new();
                    push_options.remote_callbacks(callbacks);

                    // Always push the commit as a Git tag named like the value of `base`
                    let refspec = format!("{base_commit}:refs/tags/{base}");
                    tracing::info!(refspec, "constructed push refspec");

                    mirror_remote
                        .push(&[&refspec], Some(&mut push_options))
                        .context(PushToMirrorSnafu {
                            url: &mirror_url,
                            refspec: &refspec,
                            commit: base_commit,
                        })?;

                    tracing::info!("successfully pushed base ref to mirror");
                    Some(mirror_url)
                } else {
                    tracing::warn!(
                        "this version is not mirrored, re-run with --mirror before merging into main"
                    );
                    None
                };

                tracing::info!("saving version-level configuration");
                let config = ProductVersionConfig {
                    base: base_commit,
                    mirror: mirror_url,
                };
                let config_path = ctx.version_config_path();
                if let Some(config_dir) = config_path.parent() {
                    std::fs::create_dir_all(config_dir)
                        .context(CreatePatchDirSnafu { path: config_dir })?;
                }

                let config_toml = toml::to_string_pretty(&config).context(SerializeConfigSnafu)?;
                File::create_new(&config_path)
                    .and_then(|mut f| f.write_all(config_toml.as_bytes()))
                    .context(SaveConfigSnafu { path: &config_path })?;

                tracing::info!(
                    config.path = ?config_path,
                    product = ctx.pv.product,
                    version = ctx.pv.version,
                    "created configuration for product version"
                );
            }
        },

        Cmd::PatchDir { pv } => {
            let ctx = ProductVersionContext {
                pv,
                images_repo_root,
            };
            println!("{}", ctx.patch_dir().display());
        }

        Cmd::WorktreeDir { pv } => {
            let ctx = ProductVersionContext {
                pv,
                images_repo_root,
            };
            println!("{}", ctx.worktree_root().display());
        }

        Cmd::ImagesDir => println!("{}", images_repo_root.display()),
    }

    Ok(())
}
