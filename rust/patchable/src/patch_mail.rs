use std::{
    ffi::OsString,
    fs::File,
    num::ParseIntError,
    path::{Path, PathBuf},
    process::{ExitStatus, Stdio},
    str::Utf8Error,
};

use git2::{Diff, Repository, Signature};
use snafu::{OptionExt as _, ResultExt, Snafu};
use tempfile::{tempdir, NamedTempFile};
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

use crate::utils::raw_git_cmd;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to create temporary directory"))]
    CreateTempDir { source: std::io::Error },
    #[snafu(display("failed to create temporary file"))]
    CreateTempFile { source: std::io::Error },

    #[snafu(display("failed to run git mailsplit"))]
    RunMailsplit { source: std::io::Error },
    #[snafu(display("git mailsplit exited with status code {status}"))]
    MailsplitFailed { status: ExitStatus },
    #[snafu(display("git mailsplit returned invalid UTF-8"))]
    MailsplitOutput { source: Utf8Error },
    #[snafu(display("git mailsplit invalid number"))]
    ParseMailsplit { source: ParseIntError },

    #[snafu(display(
        "failed to open mail file at {path:?} (should have been created by git mailsplit)"
    ))]
    OpenMailFile {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("failed to run git mailinfo"))]
    RunMailinfo { source: std::io::Error },
    #[snafu(display("git mailinfo exited with status code {status}"))]
    MailinfoFailed { status: ExitStatus },
    #[snafu(display("git mailsplit returned invalid UTF-8"))]
    MailinfoOutput { source: Utf8Error },
    #[snafu(display("failed to read message file created by git mailinfo"))]
    ReadMailinfoMessage { source: std::io::Error },
    #[snafu(display("failed to read patch file created by git mailinfo"))]
    ReadMailinfoPatch { source: std::io::Error },

    #[snafu(display("malformed mail header (should be separated by \": \")"))]
    MalformedMailHeader,
    #[snafu(display("unknown mail header type {header:?}"))]
    UnknownMailHeader { header: String },
    #[snafu(display("patch mail has no \"Author\" header"))]
    NoAuthorName,
    #[snafu(display("patch mail has no \"Email\" header"))]
    NoAuthorEmail,
    #[snafu(display("patch mail has no \"Date\" header"))]
    NoDate,
    #[snafu(display("patch mail has no \"Subject\" header"))]
    NoSubject,
    #[snafu(display("failed to parse \"Date\" header (should be RFC2822)"))]
    InvalidMailDate {
        #[snafu(source(from(time::error::Parse, Box::new)))]
        source: Box<time::error::Parse>,
        date: String,
    },
    #[snafu(display("failed to build commit signature from headers"))]
    InvalidSignature { source: git2::Error },
    #[snafu(display("patch has invalid diff"))]
    InvalidDiff { source: git2::Error },
}
type Result<T, E = Error> = std::result::Result<T, E>;

/// Splits a series of git patch emails into individual patch emails.
pub fn mailsplit(repo: &Repository, patch_file: &Path) -> Result<impl Iterator<Item = PathBuf>> {
    let base_dir = tempdir().context(CreateTempDirSnafu)?;
    let mailsplit = raw_git_cmd(repo)
        .arg("mailsplit")
        // From <commit> is ignored anyway, so there's no point requiring it
        .arg("-b")
        // mailsplit doesn't accept split arguments ("-o dir")
        .arg({
            let mut output_arg = OsString::from("-o");
            output_arg.push(base_dir.path());
            output_arg
        })
        .arg("--")
        .arg(patch_file)
        .stderr(Stdio::inherit())
        .output()
        .context(RunMailsplitSnafu)?;
    if !mailsplit.status.success() {
        return MailsplitFailedSnafu {
            status: mailsplit.status,
        }
        .fail();
    }
    let mailsplit_patch_count = std::str::from_utf8(&mailsplit.stdout)
        .context(MailsplitOutputSnafu)?
        .trim()
        .parse()
        .context(ParseMailsplitSnafu)?;
    Ok((1..=mailsplit_patch_count).map(move |patch_i| {
        base_dir.path().join(
            // Matches the format emitted by git-mailsplit
            format!("{patch_i:04}"),
        )
    }))
}

pub struct Mailinfo {
    headers: String,
    rest_of_message: String,
    patch: Vec<u8>,
}

pub struct ParsedPatch {
    pub subject: String,
    pub message: String,
    pub author: Signature<'static>,
    pub patch: Diff<'static>,
}

impl Mailinfo {
    pub fn parse(self) -> Result<ParsedPatch> {
        let mut author_name = None;
        let mut author_email = None;
        let mut date = None;
        let mut subject = None;
        for patch_info_line in self.headers.lines() {
            if !patch_info_line.is_empty() {
                match patch_info_line
                    .split_once(": ")
                    .context(MalformedMailHeaderSnafu)?
                {
                    ("Author", x) => author_name = Some(x),
                    ("Email", x) => author_email = Some(x),
                    ("Date", x) => date = Some(x),
                    ("Subject", x) => subject = Some(x),
                    (header, _) => return UnknownMailHeaderSnafu { header }.fail(),
                }
            }
        }
        let date = date.context(NoDateSnafu)?;
        let date = OffsetDateTime::parse(date, &Rfc2822).context(InvalidMailDateSnafu { date })?;
        let subject = subject.context(NoSubjectSnafu)?.trim();
        let full_msg = if self.rest_of_message.is_empty() {
            subject.to_string()
        } else {
            format!("{}\n\n{}", subject, self.rest_of_message.trim())
        };
        Ok(ParsedPatch {
            subject: subject.to_string(),
            message: full_msg,
            author: Signature::new(
                author_name.context(NoAuthorNameSnafu)?,
                author_email.context(NoAuthorEmailSnafu)?,
                &git2::Time::new(date.unix_timestamp(), date.offset().whole_minutes().into()),
            )
            .context(InvalidSignatureSnafu)?,
            patch: Diff::from_buffer(&self.patch).context(InvalidDiffSnafu)?,
        })
    }
}

pub fn mailinfo(repo: &Repository, patch_email_file: &Path) -> Result<Mailinfo> {
    let msg_file = NamedTempFile::new()
        .context(CreateTempFileSnafu)?
        .into_temp_path();
    let patch_file = NamedTempFile::new()
        .context(CreateTempFileSnafu)?
        .into_temp_path();
    let mailinfo = raw_git_cmd(repo)
        .arg("mailinfo")
        .args([&msg_file, &patch_file])
        .stdin(File::open(patch_email_file).context(OpenMailFileSnafu {
            path: patch_email_file,
        })?)
        .stderr(Stdio::inherit())
        .output()
        .context(RunMailinfoSnafu)?;
    if !mailinfo.status.success() {
        return MailinfoFailedSnafu {
            status: mailinfo.status,
        }
        .fail();
    }
    let patch_info = std::str::from_utf8(&mailinfo.stdout).context(MailinfoOutputSnafu)?;
    Ok(Mailinfo {
        headers: patch_info.to_string(),
        rest_of_message: std::fs::read_to_string(msg_file).context(ReadMailinfoMessageSnafu)?,
        patch: std::fs::read(patch_file).context(ReadMailinfoPatchSnafu)?,
    })
}
