//! Fetches image digests with crane and verified SBOM attestations with cosign.

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use oci_spec::image::ImageIndex;
use serde_json::Value;
use snafu::{OptionExt, ResultExt, Snafu, ensure};

use crate::expectations::ExpectationsFile;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to run {program}"))]
    Run {
        source: std::io::Error,
        program: &'static str,
    },

    #[snafu(display("failed to fetch the image index: {message}"))]
    FetchIndex { message: String },

    #[snafu(display("failed to parse the image index"))]
    ParseIndex { source: serde_json::Error },

    #[snafu(display("the tag is not a multi-arch image index"))]
    NotAnIndex,

    #[snafu(display("failed to verify the attestation: {message}"))]
    VerifyAttestation { message: String },

    #[snafu(display("failed to parse the output of cosign"))]
    ParseAttestation { source: serde_json::Error },

    #[snafu(display("the attestation has no payload"))]
    MissingPayload,

    #[snafu(display("failed to decode the payload of the attestation"))]
    DecodePayload { source: base64::DecodeError },

    #[snafu(display("failed to parse the statement of the attestation"))]
    ParseStatement { source: serde_json::Error },

    #[snafu(display("the attestation has no predicate"))]
    MissingPredicate,

    #[snafu(display("cosign returned no attestation"))]
    NoAttestation,

    #[snafu(display("failed to serialize the SBOM"))]
    SerializeSbom { source: serde_json::Error },

    #[snafu(display("failed to read the cached SBOM {path:?}"))]
    ReadCache {
        source: std::io::Error,
        path: PathBuf,
    },
}

/// The kinds of SBOM attestations, named as cosign names their predicate types.
#[derive(Debug, Clone, Copy)]
pub enum Attestation {
    CycloneDx,
    Spdx,
}

impl Attestation {
    pub fn cosign_type(self) -> &'static str {
        match self {
            Self::CycloneDx => "cyclonedx",
            Self::Spdx => "spdxjson",
        }
    }

    /// The ID of the check that reports a missing attestation.
    pub fn check(self) -> &'static str {
        match self {
            Self::CycloneDx => "attestation-cyclonedx",
            Self::Spdx => "attestation-spdxjson",
        }
    }
}

/// The digest of every architecture of a multi-arch image.
pub fn resolve_digests(
    registry: &str,
    image: &str,
    tag: &str,
) -> Result<BTreeMap<String, String>, Error> {
    let output = run("crane", &["manifest", &format!("{registry}/{image}:{tag}")])?;
    ensure!(
        output.status.success(),
        FetchIndexSnafu {
            message: last_line(&output.stderr).unwrap_or_else(|| "no output".to_owned())
        }
    );

    // A single-architecture image fails here, its manifest has no `manifests`.
    let index: ImageIndex = serde_json::from_slice(&output.stdout).context(ParseIndexSnafu)?;
    let digests: BTreeMap<String, String> = index
        .manifests()
        .iter()
        .filter_map(|manifest| {
            Some((
                manifest.platform().as_ref()?.architecture().to_string(),
                manifest.digest().to_string(),
            ))
        })
        .collect();
    ensure!(!digests.is_empty(), NotAnIndexSnafu);
    Ok(digests)
}

/// Fetches and verifies one attestation, and returns the SBOM in it as compact JSON.
///
/// cosign checks the signature against the identity of our GitHub workflows, and that the
/// attestation is for exactly this digest. Attestations are immutable per digest, so they are
/// cached by it.
pub fn fetch_sbom(
    expectations: &ExpectationsFile,
    image: &str,
    digest: &str,
    attestation: Attestation,
    cache_dir: Option<&Path>,
) -> Result<String, Error> {
    let cache = cache_dir.map(|cache_dir| {
        cache_dir.join(image).join(format!(
            "{digest}.{kind}.json",
            digest = digest.replace(':', "-"),
            kind = attestation.cosign_type(),
        ))
    });
    if let Some(cache) = &cache
        && cache.exists()
    {
        return std::fs::read_to_string(cache).context(ReadCacheSnafu { path: cache });
    }

    let reference = format!(
        "{registry}/{image}@{digest}",
        registry = expectations.registry
    );
    let arguments = [
        "verify-attestation",
        "--type",
        attestation.cosign_type(),
        "--certificate-identity-regexp",
        &expectations.certificate_identity_regexp,
        "--certificate-oidc-issuer",
        &expectations.certificate_oidc_issuer,
        &reference,
    ];
    // One retry, so a hiccup of the registry or of the network does not raise an alert.
    let mut output = run("cosign", &arguments)?;
    if !output.status.success() {
        output = run("cosign", &arguments)?;
    }
    ensure!(
        output.status.success(),
        VerifyAttestationSnafu {
            message: last_line(&output.stderr).unwrap_or_else(|| "no output".to_owned())
        }
    );

    let mut sboms = Vec::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if line.trim().is_empty() {
            continue;
        }
        let envelope: Value = serde_json::from_str(line).context(ParseAttestationSnafu)?;
        let payload = envelope
            .get("payload")
            .and_then(Value::as_str)
            .context(MissingPayloadSnafu)?;
        let statement = BASE64.decode(payload).context(DecodePayloadSnafu)?;
        let mut statement: Value =
            serde_json::from_slice(&statement).context(ParseStatementSnafu)?;
        let sbom = statement
            .get_mut("predicate")
            .map(Value::take)
            .context(MissingPredicateSnafu)?;
        sboms.push(sbom);
    }

    // A digest that was published twice has two attestations, the newest one counts.
    let sbom = sboms
        .into_iter()
        .max_by_key(|sbom| timestamp(sbom).to_owned())
        .context(NoAttestationSnafu)?;
    let json = serde_json::to_string(&sbom).context(SerializeSbomSnafu)?;

    if let Some(cache) = &cache {
        // The cache only saves time, so failing to write it is not worth a finding.
        let written = cache
            .parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .and_then(|()| std::fs::write(cache, &json));
        if let Err(error) = written {
            eprintln!("warning: failed to write the cache file {cache:?}: {error}");
        }
    }
    Ok(json)
}

fn timestamp(sbom: &Value) -> &str {
    sbom.pointer("/metadata/timestamp")
        .or_else(|| sbom.pointer("/creationInfo/created"))
        .and_then(Value::as_str)
        .unwrap_or_default()
}

fn run(program: &'static str, arguments: &[&str]) -> Result<Output, Error> {
    Command::new(program)
        .args(arguments)
        .output()
        .context(RunSnafu { program })
}

/// The last non-empty line a tool printed, which is where crane and cosign put the reason they
/// failed.
fn last_line(output: &[u8]) -> Option<String> {
    String::from_utf8_lossy(output)
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .map(ToOwned::to_owned)
}
