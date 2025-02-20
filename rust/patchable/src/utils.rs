use std::path::Path;

use git2::Repository;
use tracing::Span;
use tracing_indicatif::{span_ext::IndicatifSpanExt, style::ProgressStyle};

pub fn progress_bar_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{span_child_prefix}{spinner} {span_name}{{{span_fields}}} {wide_msg} {bar:40} {percent:>3}%",
    )
    .expect("hard-coded template should be valid")
}

/// Runs a function whenever a `value` changes "enough".
///
/// See [`Self::update`], and especially [`Self::update_span_progress`].
pub struct Quantizer {
    value: usize,
    total: usize,
    divisor: usize,
}

impl Quantizer {
    fn new(divisor: usize) -> Self {
        Self {
            divisor,
            total: 0,
            value: 0,
        }
    }

    pub fn percent() -> Self {
        Self::new(100)
    }

    fn fraction(&self, value: usize) -> usize {
        value.checked_div(self.total / self.divisor).unwrap_or(0)
    }

    /// Runs `f` if `total` has changed, or `value` has changed significantly compared to `total`.
    pub fn update(&mut self, value: usize, total: usize, f: impl FnOnce()) {
        if self.total != total
            || self.fraction(self.value) != self.fraction(value)
            || (value == total) != (self.value == total)
        {
            f();
            self.value = value;
            self.total = total;
        }
    }

    /// Updates the progress of `span`, if there is a significant change.
    ///
    /// Use this when there are many fine-grained steps, since otherwise the cost of [`Span::pb_set_position`]
    /// will dominate the cost of the actual task.
    pub fn update_span_progress(&mut self, value: usize, total: usize, span: &Span) {
        self.update(value, total, || {
            span.pb_set_length(total as u64);
            span.pb_set_position(value as u64);
        });
    }
}

/// Runs a raw git command in the environment of a Git repository.
///
/// Used for functionality that is not currently implemented by libgit2/gix.
///
/// NOTE: To avoid clobbering the terminal output, processes executed by this
/// that inherit stdout and/or stderr *must* wrap the execution of the child
/// (such as [`std::process::Command::output`] or [`std::process::Command::status`])
/// in [`tracing_indicatif::suspend_tracing_indicatif`].
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
