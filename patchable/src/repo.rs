use std::path::Path;

use git2::{FetchOptions, ObjectType, Oid, Repository, RepositoryInitOptions, WorktreeAddOptions};

/// Open the Git repository at `path`, creating it if it doesn't already exist.
#[tracing::instrument]
pub fn ensure_bare_repo(path: &Path) -> Repository {
    match Repository::open(path) {
        Ok(repo) => {
            tracing::info!("repository found, reusing");
            repo
        }
        Err(err) => {
            tracing::info!(
                error = &err as &dyn std::error::Error,
                "repository not found, initializing"
            );
            let repo = Repository::init_opts(
                path,
                RepositoryInitOptions::new()
                    .bare(true)
                    .external_template(false),
            )
            .unwrap();

            repo
        }
    }
}

/// Pull `commit` from `upstream_url`, if it doesn't already exist.
#[tracing::instrument(skip(repo))]
pub fn ensure_commit_exists_or_pull(repo: &Repository, commit: &str, upstream_url: &str) -> Oid {
    match repo.revparse_single(commit) {
        Ok(commit_obj) => {
            tracing::info!("base commit exists, reusing");
            commit_obj
        }
        Err(err) => {
            tracing::info!(
                error = &err as &dyn std::error::Error,
                "base commit not found, fetching from upstream"
            );
            repo.remote_anonymous(upstream_url)
                .unwrap()
                .fetch(
                    &[commit],
                    Some(
                        FetchOptions::new()
                            // TODO: could be 1, CLI option maybe?
                            .depth(0),
                    ),
                    Some("fetching patchable base commit"),
                )
                .unwrap();
            tracing::info!("fetched base commit");
            repo.revparse_single(commit).unwrap()
        }
    }
    .peel_to_commit()
    .unwrap()
    .id()
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
    commit: Oid,
) {
    match Repository::open(worktree_root) {
        Ok(worktree) => {
            tracing::info!("worktree found, reusing and resetting");
            // We can't reset the branch if it's already checked out, so detach to the commit instead for the meantime
            worktree
                .set_head_detached(worktree.head().unwrap().peel_to_commit().unwrap().id())
                .unwrap();
            let branch = worktree
                .branch(branch, &worktree.find_commit(commit).unwrap(), true)
                .unwrap()
                .into_reference();
            worktree
                .checkout_tree(&branch.peel(ObjectType::Commit).unwrap(), None)
                .unwrap();
            worktree.set_head_bytes(branch.name_bytes()).unwrap();
        }
        Err(err) => {
            tracing::info!(
                error = &err as &dyn std::error::Error,
                "worktree not found, creating"
            );
            std::fs::create_dir_all(worktree_root.parent().unwrap()).unwrap();
            let worktree_ref = repo
                .branch(branch, &repo.find_commit(commit).unwrap(), true)
                .unwrap()
                .into_reference();
            repo.worktree(
                worktree_name,
                worktree_root,
                Some(WorktreeAddOptions::new().reference(Some(&worktree_ref))),
            )
            .unwrap();
        }
    }
}
