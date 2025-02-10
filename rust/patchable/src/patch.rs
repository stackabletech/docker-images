use std::{
    path::{Path, PathBuf},
    process::ExitStatus,
};

use git2::{Oid, Repository};
use snafu::{OptionExt, ResultExt as _, Snafu};

use crate::{
    error::{self, CommitId},
    patch_mail::{self, mailinfo, mailsplit},
    utils::raw_git_cmd,
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to open stgit series file {path:?}"))]
    OpenStgitSeriesFile {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("failed to list contents of patch directory {path:?}"))]
    ListPatchDirectory {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to split patch file {patch_file:?}"))]
    Mailsplit {
        source: patch_mail::Error,
        patch_file: PathBuf,
    },
    #[snafu(display(
        "failed to split patch email file {patch_email_file:?} (from {patch_file:?})"
    ))]
    Mailinfo {
        source: patch_mail::Error,
        patch_email_file: PathBuf,
        patch_file: PathBuf,
    },
    #[snafu(display(
        "failed to parse patch email file {patch_email_file:?} (from {patch_file:?})"
    ))]
    ParseMailinfo {
        source: patch_mail::Error,
        patch_email_file: PathBuf,
        patch_file: PathBuf,
    },

    #[snafu(display("failed to find parent commit {commit}"))]
    FindParentCommit {
        source: git2::Error,
        commit: error::CommitId,
    },
    #[snafu(display("failed to read tree of parent commit {parent_commit}"))]
    ReadParentCommitTree {
        source: git2::Error,
        parent_commit: error::CommitId,
    },
    #[snafu(display("failed to apply patch {patch_email_file:?} (from {patch_file:?}) to parent commit {parent_commit}"))]
    ApplyPatch {
        source: git2::Error,
        parent_commit: error::CommitId,
        patch_email_file: PathBuf,
        patch_file: PathBuf,
    },
    #[snafu(display("failed to write tree for patch {patch_email_file:?} (from {patch_file:?}) applied to parent commit {parent_commit}"))]
    WritePatchedTree {
        source: git2::Error,
        parent_commit: error::CommitId,
        patch_email_file: PathBuf,
        patch_file: PathBuf,
    },
    #[snafu(display("failed to read patched tree {tree}"))]
    ReadPatchedTree { source: git2::Error, tree: Oid },
    #[snafu(display("failed to write commit for patch {patch_email_file:?} (from {patch_file:?}) applied to parent commit {parent_commit}"))]
    WriteCommit {
        source: git2::Error,
        parent_commit: error::CommitId,
        patch_email_file: PathBuf,
        patch_file: PathBuf,
    },

    #[snafu(display("failed to configure revwalk for canonicalization"))]
    ConfigureCanonicalizeRevwalk { source: git2::Error },
    #[snafu(display("revwalk returned invalid object"))]
    CanonicalizeRevwalkObject { source: git2::Error },
    #[snafu(display("failed to find commit {commit} from canonicalization revwalk"))]
    CanonicalizeRevwalkFindCommit {
        source: git2::Error,
        commit: CommitId,
    },
    #[snafu(display("failed to find read tree from original commit {commit}"))]
    CanonicalizeReadOriginalCommitTree {
        source: git2::Error,
        commit: CommitId,
    },
    #[snafu(display(
        "failed to write canonicalized commit of {original_commit} (with canonicalized parent {parent_commit})"
    ))]
    CanonicalizeWriteCommit {
        source: git2::Error,
        parent_commit: error::CommitId,
        original_commit: error::CommitId,
    },
    #[snafu(display("commit {commit}'s commit message is invalid UTF-8"))]
    NonUtf8CommitMessage { commit: CommitId },

    #[snafu(display("failed to delete old patch file {path:?}"))]
    DeleteOldPatch {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("failed to run git format-mail"))]
    RunFormatMail { source: std::io::Error },
    #[snafu(display("git format-mail exited with status code {status}"))]
    FormatMailFailed { status: ExitStatus },
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
                .and_then(|entries| {
                    entries
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
                })
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
    for ref patch_file in patch_files(patch_dir)? {
        tracing::info!(
            patch.file = ?patch_file,
            "parsing patch"
        );
        for ref patch_email_file in
            mailsplit(repo, patch_file).context(MailsplitSnafu { patch_file })?
        {
            let patch = mailinfo(repo, patch_email_file)
                .context(MailinfoSnafu {
                    patch_email_file,
                    patch_file,
                })?
                .parse()
                .context(ParseMailinfoSnafu {
                    patch_email_file,
                    patch_file,
                })?;
            tracing::info!(
                commit.base = %last_commit_id,
                commit.subject = patch.subject,
                "applying patch"
            );
            let parent_commit =
                &repo
                    .find_commit(last_commit_id)
                    .context(FindParentCommitSnafu {
                        commit: last_commit_id,
                    })?;
            let patch_tree_id = repo
                .apply_to_tree(
                    &parent_commit
                        .tree()
                        .context(ReadParentCommitTreeSnafu { parent_commit })?,
                    &patch.patch,
                    None,
                )
                .context(ApplyPatchSnafu {
                    parent_commit,
                    patch_email_file,
                    patch_file,
                })?
                .write_tree_to(repo)
                .context(WritePatchedTreeSnafu {
                    parent_commit,
                    patch_email_file,
                    patch_file,
                })?;
            last_commit_id = repo
                .commit(
                    None,
                    &patch.author,
                    &patch.author,
                    &patch.message,
                    &repo
                        .find_tree(patch_tree_id)
                        .context(ReadPatchedTreeSnafu {
                            tree: patch_tree_id,
                        })?,
                    &[parent_commit],
                )
                .context(WriteCommitSnafu {
                    parent_commit,
                    patch_email_file,
                    patch_file,
                })?;
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
pub fn canonicalize_commit_history(
    repo: &Repository,
    base_commit: Oid,
    leaf_commit: Oid,
) -> Result<Oid> {
    tracing::info!("canonicalizing commit history");
    let canonicalize_revwalk = repo
        .revwalk()
        .and_then(|mut walk| {
            walk.push(leaf_commit)?;
            walk.hide(base_commit)?;
            walk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE)?;
            Ok(walk)
        })
        .context(ConfigureCanonicalizeRevwalkSnafu)?;
    let mut last_canonical_commit = base_commit;
    for original in canonicalize_revwalk {
        let original = original.context(CanonicalizeRevwalkObjectSnafu)?;
        let original = &repo
            .find_commit(original)
            .context(CanonicalizeRevwalkFindCommitSnafu { commit: original })?;
        let author = original.author();
        last_canonical_commit = repo
            .commit(
                None,
                &author,
                &author,
                original
                    .message()
                    .context(NonUtf8CommitMessageSnafu { commit: original })?
                    .trim(),
                &original
                    .tree()
                    .context(CanonicalizeReadOriginalCommitTreeSnafu { commit: original })?,
                &[&repo
                    .find_commit(last_canonical_commit)
                    .context(FindParentCommitSnafu {
                        commit: last_canonical_commit,
                    })?],
            )
            .context(CanonicalizeWriteCommitSnafu {
                parent_commit: last_canonical_commit,
                original_commit: original,
            })?;
    }
    tracing::info!(
        leaf_commit.canonical = %last_canonical_commit,
        "canonicalized commit history"
    );
    Ok(last_canonical_commit)
}

/// Formats the commits between `base_commit` (exclusive) and `leaf_commit` (inclusive) as patches in `patch_dir`.
///
/// Deletes any existing patch files in `patch_dir`.
#[tracing::instrument(skip(repo))]
pub fn format_patches(
    repo: &Repository,
    patch_dir: &Path,
    base_commit: Oid,
    leaf_commit: Oid,
) -> Result<()> {
    tracing::info!("deleting existing patch files");
    // git format-patch is happy to overwrite existing files,
    // but we also want to delete removed (or renamed) patch files.
    for entry in patch_dir
        .read_dir()
        .context(ListPatchDirectorySnafu { path: patch_dir })?
    {
        let path = &entry
            .context(ListPatchDirectorySnafu { path: patch_dir })?
            .path();
        // git format-patch emits the mailbox format, not stgit(/quilt),
        // so also remove markers that make it look like that
        if path.file_name().is_some_and(|x| x == "series")
            || path.extension().is_some_and(|x| x == "patch")
        {
            std::fs::remove_file(path).context(DeleteOldPatchSnafu { path })?;
        }
    }

    tracing::info!("exporting commits since base");
    let status = raw_git_cmd(repo)
        .arg("format-patch")
        .arg(format!("{base_commit}..{leaf_commit}"))
        .arg("-o")
        .arg(patch_dir)
        .arg("--keep-subject")
        .status()
        .context(RunFormatMailSnafu)?;
    if !status.success() {
        return FormatMailFailedSnafu { status }.fail();
    }

    Ok(())
}
