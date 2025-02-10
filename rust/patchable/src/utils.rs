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

/// Implements (equivalents of) the [`serde`] traits over [`git2::Oid`].
///
/// For use with `#[serde(with = ...)]`.
pub mod oid_serde {
    use git2::Oid;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(value: &Oid, ser: S) -> Result<S::Ok, S::Error> {
        value.to_string().serialize(ser)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Oid, D::Error> {
        String::deserialize(de)
            .and_then(|oid| Oid::from_str(&oid).map_err(<D::Error as serde::de::Error>::custom))
    }
}
