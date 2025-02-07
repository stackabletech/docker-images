use std::{fs::File, path::Path, process::Stdio};

use git2::{Diff, Oid, Repository, Signature};
use tempfile::tempdir;
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

use crate::utils::raw_git_cmd;

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
pub fn apply_patches(repo: &Repository, patch_dir: &Path, base_commit: Oid) -> Oid {
    tracing::info!(
        patch.dir = %patch_dir.display(),
        "applying patches"
    );
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
    let mut last_commit_id = base_commit;
    for patch_file in patch_files {
        tracing::info!(
            patch.file = %patch_file.display(),
            "parsing patch"
        );
        let mailsplit_dir = tempdir().unwrap();
        let mailsplit = raw_git_cmd(repo)
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
        let mailsplit_patch_count = std::str::from_utf8(&mailsplit.stdout)
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
            let mailinfo = raw_git_cmd(repo)
                .arg("mailinfo")
                .args([&patch_split_msg_file, &patch_split_patch_file])
                .stdin(File::open(mailsplit_dir.path().join(patch_mail_file_name)).unwrap())
                .stderr(Stdio::inherit())
                .output()
                .unwrap();
            if !mailinfo.status.success() {
                panic!("failed to apply patches");
            }
            let patch_info = std::str::from_utf8(&mailinfo.stdout).unwrap();

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
                &git2::Time::new(date.unix_timestamp(), date.offset().whole_minutes().into()),
            )
            .unwrap();
            let parent_commit = repo.find_commit(last_commit_id).unwrap();
            tracing::info!(
                commit.base = %parent_commit.id(),
                commit.subject = subject.unwrap(),
                "applying patch"
            );
            let patch_tree_id = repo
                .apply_to_tree(
                    &parent_commit.tree().unwrap(),
                    &Diff::from_buffer(&std::fs::read(patch_split_patch_file).unwrap()).unwrap(),
                    None,
                )
                .unwrap()
                .write_tree_to(repo)
                .unwrap();
            let msg = std::fs::read_to_string(patch_split_msg_file).unwrap();
            let full_msg = if msg.is_empty() {
                subject.unwrap()
            } else {
                &format!("{}\n\n{msg}", subject.unwrap())
            };
            last_commit_id = repo
                .commit(
                    None,
                    &author,
                    &author,
                    full_msg.trim(),
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
    last_commit_id
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
