use std::path::Path;

use git2::Repository;

/// Runs a raw git command in the environment of a Git repository.
///
/// Used for functionality that is not currently implemented by libgit2/gix.
pub fn raw_git_cmd(repo: &Repository) -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    cmd.env("GIT_DIR", repo.path());
    cmd.env(
        "GIT_WORK_TREE",
        repo.workdir().unwrap_or(Path::new("/dev/null")),
    );
    cmd
}
