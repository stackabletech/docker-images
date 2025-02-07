use std::path::{Path, PathBuf};

use git2::{Oid, Repository};
use snafu::{ResultExt as _, Snafu};

use crate::{
    patch_mail::{mailinfo, mailsplit},
    utils::raw_git_cmd,
};

#[derive(Debug, Snafu)]
pub enum Error {
    OpenStgitSeriesFile {
        source: std::io::Error,
        path: PathBuf,
    },
    ListPatchDirectory {
        source: std::io::Error,
        path: PathBuf,
    },
}
type Result<T, E = Error> = std::result::Result<T, E>;

/// Lists the patches to apply in `patch_dir`, in order.
fn patch_files(patch_dir: &Path) -> Result<Vec<PathBuf>> {
    let series_file = patch_dir.join("series");
    Ok(match std::fs::read_to_string(&series_file) {
        Ok(file) => {
            tracing::info!(
                patch.series = %series_file.display(),
                "series file found, treating as stgit series"
            );
            file.lines()
                .map(|file_name| patch_dir.join(file_name))
                .collect::<Vec<_>>()
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            tracing::info!(
                error = &err as &dyn std::error::Error,
                patch.series = %series_file.display(),
                "series file not found, treating as git mailbox"
            );
            let mut patch_files = patch_dir
                .read_dir()
                .unwrap()
                .filter_map(|e| {
                    e.map(|entry| {
                        let path = entry.path();
                        path.extension()
                            .is_some_and(|ext| ext == "patch")
                            .then_some(path)
                    })
                    .transpose()
                })
                .collect::<Result<Vec<_>, _>>()
                .context(ListPatchDirectorySnafu { path: patch_dir })?;
            patch_files.sort();
            patch_files
        }
        Err(err) => return Err(err).context(OpenStgitSeriesFileSnafu { path: series_file }),
    })
}

/// Apply the patches in `patch_dir` to `base_commit`.
///
/// Does not modify the worktree or branch(es). Use [`ensure_worktree_is_at`] to check out the commit afterwards.
///
/// This effectively reimplements git-am, but lets us:
/// 1. Fake the committer information to match author (to ensure that we get deterministic committer IDs across machines)
/// 2. Avoid touching the worktree until all patches have been applied
///   (letting us keep any dirty files in the worktree that don't conflict with the final switcheroo,
///   even if those files are modified by some of the patches)
#[tracing::instrument(skip(repo))]
pub fn apply_patches(repo: &Repository, patch_dir: &Path, base_commit: Oid) -> Result<Oid> {
    tracing::info!(
        patch.dir = %patch_dir.display(),
        "applying patches"
    );
    let mut last_commit_id = base_commit;
    for patch_file in patch_files(patch_dir)? {
        tracing::info!(
            patch.file = ?patch_file,
            "parsing patch"
        );
        for patch_email_file in mailsplit(repo, &patch_file).unwrap() {
            let patch = mailinfo(repo, &patch_email_file).unwrap().parse().unwrap();
            tracing::info!(
                commit.base = %last_commit_id,
                commit.subject = patch.subject,
                "applying patch"
            );
            let parent_commit = repo.find_commit(last_commit_id).unwrap();
            let patch_tree_id = repo
                .apply_to_tree(&parent_commit.tree().unwrap(), &patch.patch, None)
                .unwrap()
                .write_tree_to(repo)
                .unwrap();
            last_commit_id = repo
                .commit(
                    None,
                    &patch.author,
                    &patch.author,
                    &patch.message,
                    &repo.find_tree(patch_tree_id).unwrap(),
                    &[&parent_commit],
                )
                .unwrap();
            tracing::info!(
                commit.id = %last_commit_id,
                "applied patch"
            );
        }
    }
    Ok(last_commit_id)
}

/// Canonicalize commits for all commits between `base_commit` (exclusive) and `leaf_commit` (inclusive).
///
/// Does not modify the worktree.
///
/// This should generate the same commit IDs we would have loaded in [`apply_patches`].
///
/// This is required to avoid commit ID churn because of author/committer mismatch
/// (generated whenever a commit is modified after the initial commit), and
/// whitespace mangling in git-format-patch.
#[tracing::instrument(skip(repo))]
pub fn canonicalize_commit_history(repo: &Repository, base_commit: Oid, leaf_commit: Oid) -> Oid {
    tracing::info!("canonicalizing commit history");
    let mut canonicalize_revwalk = repo.revwalk().unwrap();
    canonicalize_revwalk.push(leaf_commit).unwrap();
    canonicalize_revwalk.hide(base_commit).unwrap();
    canonicalize_revwalk
        .set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE)
        .unwrap();
    let mut last_canonical_commit = base_commit;
    for original in canonicalize_revwalk {
        let original = repo.find_commit(original.unwrap()).unwrap();
        let author = original.author();
        last_canonical_commit = repo
            .commit(
                None,
                &author,
                &author,
                original.message().unwrap().trim(),
                &original.tree().unwrap(),
                &[&repo.find_commit(last_canonical_commit).unwrap()],
            )
            .unwrap();
    }
    tracing::info!(
        leaf_commit.canonical = %last_canonical_commit,
        "canonicalized commit history"
    );
    last_canonical_commit
}

/// Formats the commits between `base_commit` (exclusive) and `leaf_commit` (inclusive) as patches in `patch_dir`.
///
/// Deletes any existing patch files in `patch_dir`.
#[tracing::instrument(skip(repo))]
pub fn format_patches(repo: &Repository, patch_dir: &Path, base_commit: Oid, leaf_commit: Oid) {
    tracing::info!("deleting existing patch files");
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

    tracing::info!("exporting commits since base");
    if !raw_git_cmd(repo)
        .arg("format-patch")
        .arg(format!("{base_commit}..{leaf_commit}"))
        .arg("-o")
        .arg(patch_dir)
        .arg("--keep-subject")
        .status()
        .unwrap()
        .success()
    {
        panic!("failed to format patches");
    }
}
