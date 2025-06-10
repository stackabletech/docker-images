use std::path::{self, Path, PathBuf};

use git2::{FetchOptions, ObjectType, Oid, Repository, RepositoryInitOptions, WorktreeAddOptions};
use snafu::{ResultExt, Snafu};

use crate::{
    error::{self, CommitRef},
    utils::{setup_git_credentials, setup_progress_tracking},
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to init repository at {path:?}"))]
    Init { source: git2::Error, path: PathBuf },
    #[snafu(display("failed to open repository at {path:?}"))]
    Open { source: git2::Error, path: PathBuf },

    #[snafu(display("failed to absolutize path {path:?}"))]
    Absolutize {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("no .patchable file found in parent of {path:?}"))]
    NoDotPatchableFound { path: PathBuf },
    #[snafu(display("failed to check if {path:?} contains a .patchable file"))]
    CheckDotPatchable {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display(
        "failed to create worktree branch {branch:?} pointing at {commit} in {repo}"
    ))]
    CreateWorktreeBranch {
        source: git2::Error,
        repo: error::RepoPath,
        branch: String,
        commit: error::CommitId,
    },
    #[snafu(display("failed to create worktree parent folder at {path:?}"))]
    CreateWorktreePath {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display(
        "failed to create worktree {path:?} pointing at {branch} in {repo} (hint: {})",
        "it may have been created but deleted in the past, try `git -C {repo} worktree prune`"
    ))]
    CreateWorktree {
        source: git2::Error,
        repo: error::RepoPath,
        path: PathBuf,
        branch: CommitRef,
    },

    #[snafu(display("failed to detach worktree {worktree} from {old_target} to {commit}"))]
    DetachWorktree {
        source: git2::Error,
        worktree: error::RepoPath,
        old_target: error::CommitRef,
        commit: error::CommitId,
    },
    #[snafu(display("failed to checkout commit {commit} to worktree {worktree}"))]
    CheckoutWorktree {
        source: git2::Error,
        worktree: error::RepoPath,
        commit: error::CommitId,
    },
    #[snafu(display("failed to update worktree {worktree}'s head to {target}"))]
    UpdateWorktreeHead {
        source: git2::Error,
        worktree: error::RepoPath,
        target: error::CommitRef,
    },

    #[snafu(display("failed to find commit {commit} in {repo}"))]
    FindCommit {
        source: git2::Error,
        repo: error::RepoPath,
        commit: error::CommitRef,
    },

    #[snafu(display("failed to create remote in {repo} for {url:?}"))]
    CreateRemote {
        source: git2::Error,
        repo: error::RepoPath,
        url: String,
    },
    #[snafu(display("failed to fetch refs {refs:?} from {url:?} to {repo}"))]
    Fetch {
        source: git2::Error,
        repo: error::RepoPath,
        url: String,
        refs: Vec<String>,
    },
}
type Result<T, E = Error> = std::result::Result<T, E>;

/// Open the Git repository at `path`, creating it if it doesn't already exist.
#[tracing::instrument]
pub fn ensure_bare_repo(path: &Path) -> Result<Repository> {
    match Repository::open(path) {
        Ok(repo) => {
            tracing::info!("repository found, reusing");
            Ok(repo)
        }
        Err(err) if err.code() == git2::ErrorCode::NotFound => {
            tracing::info!(
                error = &err as &dyn std::error::Error,
                "repository not found, initializing"
            );
            Repository::init_opts(
                path,
                RepositoryInitOptions::new()
                    .bare(true)
                    .external_template(false),
            )
            .context(InitSnafu { path })
        }
        Err(err) => Err(err).context(OpenSnafu { path }),
    }
}

/// Try to resolve and fetch `commitish` from `upstream_url`.
///
/// As an optimization, it can skip fetching if `commitish` is a literal commit ID that exists locally.
///
/// Returns the resolved commit ID.
#[tracing::instrument(skip(repo))]
pub fn resolve_and_fetch_commitish(
    repo: &Repository,
    commitish: &str,
    upstream_url: &str,
) -> Result<Oid> {
    let oid = Oid::from_str(commitish);
    let commitish_is_oid = oid.is_ok();
    let local_commit = oid.and_then(|oid| repo.find_commit(oid));
    let commit = match local_commit {
        Ok(commit_obj) => {
            tracing::info!("literal commit exists locally, reusing");
            Ok(commit_obj)
        }
        Err(err) if !commitish_is_oid || err.code() == git2::ErrorCode::NotFound => {
            tracing::info!(
                error = &err as &dyn std::error::Error,
                "base commit not found locally, fetching from upstream"
            );

            let (span_recv, mut quant_recv) =
                setup_progress_tracking(tracing::info_span!("receiving"));
            let (span_index, mut quant_index) =
                setup_progress_tracking(tracing::info_span!("indexing"));

            let _ = span_recv.enter();
            let _ = span_index.enter();

            let mut callbacks = setup_git_credentials();
            callbacks.transfer_progress(move |progress| {
                quant_recv.update_span_progress(
                    progress.received_objects(),
                    progress.total_objects(),
                    &span_recv,
                );
                quant_index.update_span_progress(
                    progress.indexed_objects(),
                    progress.total_objects(),
                    &span_index,
                );
                true
            });
            repo.remote_anonymous(upstream_url)
                .context(CreateRemoteSnafu {
                    repo,
                    url: upstream_url,
                })?
                .fetch(
                    &[commitish],
                    Some(
                        FetchOptions::new()
                            .update_fetchhead(true)
                            .remote_callbacks(callbacks)
                            // TODO: could be 1, CLI option maybe?
                            .depth(0),
                    ),
                    Some("fetching patchable base commit"),
                )
                .with_context(|_| FetchSnafu {
                    repo,
                    url: upstream_url,
                    refs: vec![commitish.to_string()],
                })?;
            tracing::info!("fetched base commit");
            // FETCH_HEAD is written by Remote::fetch to be the last reference fetched
            repo.revparse_single("FETCH_HEAD")
                .and_then(|obj| obj.peel_to_commit())
        }
        Err(err) => Err(err),
    }
    .context(FindCommitSnafu {
        repo,
        commit: commitish,
    })?;
    Ok(commit.id())
}

/// Ensure that the worktree at `worktree_root` exists and is checked out at `branch`.
///
/// The worktree will be created if necessary, and the branch will be created or reset to `commit`.
#[tracing::instrument(skip(repo))]
pub fn ensure_worktree_is_at(
    repo: &Repository,
    worktree_name: &str,
    worktree_root: &Path,
    branch: &str,
    base_branch: Option<&str>,
    commit: Oid,
) -> Result<()> {
    tracing::info!("checking out worktree");
    match Repository::open(worktree_root) {
        Ok(worktree) => {
            tracing::info!("worktree found, reusing");
            let commit_obj = worktree
                .find_commit(commit)
                .context(FindCommitSnafu { repo, commit })?;
            // We can't reset the branch if it's already checked out, so detach to the commit instead for the meantime
            if let Ok(head) = worktree.head() {
                tracing::info!(head.old = head.name(), "detaching worktree head");
                let head_commit = head
                    .peel_to_commit()
                    .context(FindCommitSnafu {
                        repo: &worktree,
                        commit: &head,
                    })?
                    .id();
                worktree
                    .set_head_detached(head_commit)
                    .context(DetachWorktreeSnafu {
                        worktree: worktree_root,
                        old_target: head,
                        commit: head_commit,
                    })?;
            }
            let mut branch =
                worktree
                    .branch(branch, &commit_obj, true)
                    .context(CreateWorktreeBranchSnafu {
                        repo: &worktree,
                        branch,
                        commit,
                    })?;
            // Set the base branch as the patch branch's upstream, this helps some git GUIs (like magit)
            // visualize the difference between upstream and our patchset.
            if let Err(err) = branch.set_upstream(base_branch) {
                tracing::warn!(
                    error = &err as &dyn std::error::Error,
                    branch.base = base_branch,
                    "failed to set upstream branch, ignoring..."
                );
            }
            let branch_ref = branch.into_reference();
            let commit = branch_ref
                .peel(ObjectType::Commit)
                .context(FindCommitSnafu {
                    repo: &worktree,
                    commit: &branch_ref,
                })?;
            worktree
                .checkout_tree(&commit, None)
                .context(CheckoutWorktreeSnafu {
                    worktree: &worktree,
                    commit: &commit,
                })?;
            worktree
                .set_head_bytes(branch_ref.name_bytes())
                .context(UpdateWorktreeHeadSnafu {
                    worktree: &worktree,
                    target: &branch_ref,
                })?;
            Ok(())
        }
        Err(err) if err.code() == git2::ErrorCode::NotFound => {
            tracing::info!(
                error = &err as &dyn std::error::Error,
                "worktree not found, creating"
            );
            if let Some(parent) = worktree_root.parent() {
                std::fs::create_dir_all(parent)
                    .context(CreateWorktreePathSnafu { path: parent })?;
            }
            let worktree_ref = repo
                .branch(
                    branch,
                    &repo
                        .find_commit(commit)
                        .context(FindCommitSnafu { repo, commit })?,
                    true,
                )
                .context(CreateWorktreeBranchSnafu {
                    repo,
                    branch,
                    commit,
                })?
                .into_reference();
            repo.worktree(
                worktree_name,
                worktree_root,
                Some(WorktreeAddOptions::new().reference(Some(&worktree_ref))),
            )
            .context(CreateWorktreeSnafu {
                repo,
                path: worktree_root,
                branch: worktree_ref,
            })?;
            Ok(())
        }
        Err(err) => Err(err).context(OpenSnafu {
            path: worktree_root,
        }),
    }
}

/// Try to find the Git images repository in a parent directory of `path` (or `path` itself).
///
/// The repository is detected by that it contains a file named `.patchable`.
pub fn discover_images_repo(path: impl AsRef<Path>) -> Result<Repository> {
    let full_path = path::absolute(&path).context(AbsolutizeSnafu {
        path: path.as_ref(),
    })?;
    let mut path: &Path = &full_path;
    loop {
        match path.join(".patchable").try_exists() {
            Ok(true) => break Repository::open(path).context(OpenSnafu { path }),
            Ok(false) => match path.parent() {
                Some(p) => path = p,
                None => break NoDotPatchableFoundSnafu { path: full_path }.fail(),
            },
            Err(err) => break Err(err).context(CheckDotPatchableSnafu { path }),
        }
    }
}
