use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Stdio,
};

use git2::{Diff, Repository, Signature};
use snafu::Snafu;
use tempfile::{tempdir, NamedTempFile};
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

use crate::utils::raw_git_cmd;

#[derive(Debug, Snafu)]
pub enum Error {}
type Result<T, E = Error> = std::result::Result<T, E>;

/// Splits a series of git patch emails into individual patch emails.
pub fn mailsplit(repo: &Repository, patch_file: &Path) -> Result<impl Iterator<Item = PathBuf>> {
    let base_dir = tempdir().unwrap();
    let mailsplit = raw_git_cmd(repo)
        .arg("mailsplit")
        // mailsplit doesn't accept split arguments ("-o dir")
        .arg(format!("-o{}", base_dir.path().to_str().unwrap()))
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
        .parse()
        .unwrap();
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
        let full_msg = if self.rest_of_message.is_empty() {
            subject.unwrap().trim().to_string()
        } else {
            format!(
                "{}\n\n{}",
                subject.unwrap().trim(),
                self.rest_of_message.trim()
            )
        };
        Ok(ParsedPatch {
            subject: subject.unwrap().trim().to_string(),
            message: full_msg,
            author: Signature::new(
                author_name.unwrap(),
                author_email.unwrap(),
                &git2::Time::new(date.unix_timestamp(), date.offset().whole_minutes().into()),
            )
            .unwrap(),
            patch: Diff::from_buffer(&self.patch).unwrap(),
        })
    }
}

pub fn mailinfo(repo: &Repository, patch_email_file: &Path) -> Result<Mailinfo> {
    let msg_file = NamedTempFile::new().unwrap().into_temp_path();
    let patch_file = NamedTempFile::new().unwrap().into_temp_path();
    let mailinfo = raw_git_cmd(repo)
        .arg("mailinfo")
        .args([&msg_file, &patch_file])
        .stdin(File::open(patch_email_file).unwrap())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
    if !mailinfo.status.success() {
        panic!("failed to apply patches");
    }
    let patch_info = std::str::from_utf8(&mailinfo.stdout).unwrap();
    Ok(Mailinfo {
        headers: patch_info.to_string(),
        rest_of_message: std::fs::read_to_string(msg_file).unwrap(),
        patch: std::fs::read(patch_file).unwrap(),
    })
}
