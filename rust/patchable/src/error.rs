//! Error type helpers.

use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use git2::{Commit, Object, Oid, Reference, Repository};

#[derive(Debug)]
pub struct CommitId(Box<Oid>);
impl Display for CommitId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<Oid> for CommitId {
    fn from(value: Oid) -> Self {
        Self(Box::new(value))
    }
}
impl From<&Commit<'_>> for CommitId {
    fn from(value: &Commit<'_>) -> Self {
        value.id().into()
    }
}
impl From<&Object<'_>> for CommitId {
    fn from(value: &Object<'_>) -> Self {
        value.id().into()
    }
}
impl From<Object<'_>> for CommitId {
    fn from(value: Object<'_>) -> Self {
        (&value).into()
    }
}

#[derive(Debug)]
pub struct CommitRef(String);
impl Display for CommitRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl From<&str> for CommitRef {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
impl From<Oid> for CommitRef {
    fn from(value: Oid) -> Self {
        Self(value.to_string())
    }
}
impl From<Object<'_>> for CommitRef {
    fn from(value: Object<'_>) -> Self {
        value.id().into()
    }
}
impl From<&Reference<'_>> for CommitRef {
    fn from(value: &Reference<'_>) -> Self {
        value.name().unwrap_or("<invalid ref>").into()
    }
}
impl From<Reference<'_>> for CommitRef {
    fn from(value: Reference<'_>) -> Self {
        (&value).into()
    }
}

#[derive(Debug)]
pub struct RepoPath(PathBuf);
impl Display for RepoPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl From<&Path> for RepoPath {
    fn from(value: &Path) -> Self {
        Self(value.into())
    }
}
impl From<&Repository> for RepoPath {
    fn from(value: &Repository) -> Self {
        value.path().into()
    }
}
