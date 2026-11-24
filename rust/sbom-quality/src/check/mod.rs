//! The checks, their findings and what the checks of both formats share.

mod cyclonedx;
mod spdx;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    expectations::{Expectations, Target},
    purl::{Purl, percent_decode, percent_encode},
    sbom::{CycloneDx, Spdx, remove_nulls},
};

pub const ORGANIZATION: &str = "Stackable GmbH";

/// Share of components that may differ between the architectures of an image.
const MAX_ARCHITECTURE_DIFFERENCE: f64 = 0.05;

/// The components of an image as (purl type, name), which is the same on every architecture.
///
/// Not the version: the architectures are built by separate jobs, and an RPM update that is
/// released in between ends up in only one of them.
pub type Identity = BTreeSet<(String, String)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Serialize)]
pub struct Finding {
    pub severity: Severity,

    /// ID of the check, which `ignore` in the expectations refers to.
    pub check: &'static str,

    pub message: String,
}

/// Numbers collected while checking, for `--suggest-floors` and the JSON output.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Stats {
    pub bytes: Option<usize>,
    pub license_coverage: Option<f64>,
    pub components: BTreeMap<String, u64>,

    /// Paths of the build-time SBOMs and the number of components taken from each.
    pub embedded_sboms: BTreeMap<String, u64>,

    pub direct_dependencies: Option<usize>,
}

/// The outcome of checking one image architecture.
#[derive(Debug)]
pub struct ImageResult {
    pub target: Target,
    pub digest: Option<String>,
    pub findings: Vec<Finding>,
    pub stats: Stats,

    /// The number of findings dropped because the expectations ignore their check.
    pub ignored: usize,
}

impl ImageResult {
    pub fn new(target: Target, digest: Option<String>) -> Self {
        Self {
            target,
            digest,
            findings: Vec::new(),
            stats: Stats::default(),
            ignored: 0,
        }
    }

    pub fn error(&mut self, check: &'static str, message: impl Into<String>) {
        self.findings.push(Finding {
            severity: Severity::Error,
            check,
            message: message.into(),
        });
    }

    pub fn warning(&mut self, check: &'static str, message: impl Into<String>) {
        self.findings.push(Finding {
            severity: Severity::Warning,
            check,
            message: message.into(),
        });
    }

    pub fn has_errors(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == Severity::Error)
    }
}

/// Runs every check on the SBOMs of one image architecture, given as JSON. A missing SBOM is
/// skipped, the caller reports why it is missing.
pub fn check_sboms(
    result: &mut ImageResult,
    cyclonedx: Option<&str>,
    spdx: Option<&str>,
    expectations: &Expectations,
) -> Identity {
    let cyclonedx_purls = cyclonedx.and_then(|json| {
        let (bom, size) = parse::<CycloneDx>(result, "cdx-format", "CycloneDX", json)?;
        cyclonedx::check_schema(result, json, &bom);
        Some(cyclonedx::check(result, &bom, size, expectations))
    });
    let spdx_purls = spdx
        .and_then(|json| parse::<Spdx>(result, "spdx-format", "SPDX", json))
        .map(|(document, _)| spdx::check(result, &document, &expectations.registry));

    if let (Some(cyclonedx_purls), Some(spdx_purls)) = (&cyclonedx_purls, &spdx_purls)
        && !cyclonedx_purls.is_empty()
        && !spdx_purls.is_empty()
    {
        check_formats_agree(result, cyclonedx_purls, spdx_purls);
    }

    cyclonedx_purls
        .iter()
        .flatten()
        .filter_map(|purl| Purl::parse(purl))
        .map(|purl| (purl.package_type, purl.name))
        .collect()
}

/// Parses an SBOM. Returns it with its size as compact JSON, or reports why it is unreadable.
fn parse<T: DeserializeOwned>(
    result: &mut ImageResult,
    check: &'static str,
    format: &str,
    json: &str,
) -> Option<(T, usize)> {
    let mut value: Value = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(error) => {
            result.error(
                check,
                format!("the {format} SBOM is not valid JSON: {error}"),
            );
            return None;
        }
    };
    let size = serde_json::to_string(&value).map_or(json.len(), |compact| compact.len());

    remove_nulls(&mut value);
    match serde_json::from_value(value) {
        Ok(document) => Some((document, size)),
        Err(error) => {
            result.error(
                check,
                format!("the {format} SBOM does not have the expected structure: {error}"),
            );
            None
        }
    }
}

/// Both documents are written from the same data, so they have to list the same packages.
fn check_formats_agree(
    result: &mut ImageResult,
    cyclonedx_purls: &BTreeSet<String>,
    spdx_purls: &BTreeSet<String>,
) {
    let only_cyclonedx: Vec<&String> = cyclonedx_purls.difference(spdx_purls).collect();
    let only_spdx: Vec<&String> = spdx_purls.difference(cyclonedx_purls).collect();
    if !only_cyclonedx.is_empty() {
        result.error(
            "spdx-matches-cyclonedx",
            format!(
                "{count} purls are only in the CycloneDX SBOM, e.g. {examples}",
                count = only_cyclonedx.len(),
                examples = examples(only_cyclonedx),
            ),
        );
    }
    if !only_spdx.is_empty() {
        result.error(
            "spdx-matches-cyclonedx",
            format!(
                "{count} purls are only in the SPDX SBOM, e.g. {examples}",
                count = only_spdx.len(),
                examples = examples(only_spdx),
            ),
        );
    }
}

/// The architectures of an image are built from the same sources.
///
/// A build-time SBOM that is lost on one architecture only easily stays above the component
/// floors, but not below this.
pub fn check_architectures_agree(
    results: &mut [ImageResult],
    identities: &BTreeMap<Target, Identity>,
) {
    let mut by_tag = BTreeMap::<(String, String), Vec<usize>>::new();
    for (index, result) in results.iter().enumerate() {
        if identities.contains_key(&result.target) {
            by_tag
                .entry((result.target.image.clone(), result.target.tag.clone()))
                .or_default()
                .push(index);
        }
    }

    for group in by_tag.values() {
        let Some((first, others)) = group.split_first() else {
            continue;
        };
        let (Some(first_result), Some(mine)) = (
            results.get(*first),
            results
                .get(*first)
                .and_then(|result| identities.get(&result.target)),
        ) else {
            continue;
        };
        let first_architecture = first_result.target.architecture.clone();

        for other in others {
            let Some(theirs) = results
                .get(*other)
                .and_then(|result| identities.get(&result.target))
            else {
                continue;
            };
            let union = mine.union(theirs).count();
            let difference = share(mine.symmetric_difference(theirs).count(), union);
            if difference <= MAX_ARCHITECTURE_DIFFERENCE {
                continue;
            }

            let describe = |components: Vec<&(String, String)>| {
                if components.is_empty() {
                    "nothing".to_owned()
                } else {
                    examples(
                        components
                            .iter()
                            .map(|(kind, name)| format!("{kind}:{name}")),
                    )
                }
            };
            let message = format!(
                "{difference} of the components differ from {first_architecture} (allowed: \
                 {allowed}). Only here: {only_here}. Only {first_architecture}: {only_first}",
                difference = percent(difference),
                allowed = percent(MAX_ARCHITECTURE_DIFFERENCE),
                only_here = describe(theirs.difference(mine).collect()),
                only_first = describe(mine.difference(theirs).collect()),
            );
            if let Some(result) = results.get_mut(*other) {
                result.error("architectures-agree", message);
            }
        }
    }
}

/// Drops the findings of the checks the expectations ignore.
pub fn apply_ignores(result: &mut ImageResult, expectations: &Expectations) {
    let before = result.findings.len();
    result
        .findings
        .retain(|finding| !expectations.ignore.contains(finding.check));
    result.ignored = before - result.findings.len();
}

/// Replaces `{version}`, `{release}` and `{image}` in a purl or path from the expectations.
pub fn substitute(pattern: &str, target: &Target, release: &str) -> String {
    pattern
        .replace("{version}", target.version.as_deref().unwrap_or_default())
        .replace("{release}", release)
        .replace("{image}", &target.image)
}

/// The purl the publish-image action gives the image.
pub fn expected_image_purl(registry: &str, target: &Target, digest: &str) -> String {
    format!(
        "pkg:oci/{image}@{digest}?arch={architecture}&repository_url={repository}",
        image = target.image,
        digest = percent_encode(digest),
        architecture = target.architecture,
        repository = percent_encode(&format!("{registry}/{image}", image = target.image)),
    )
}

/// Whether a component version is the expected version, with or without a suffix like
/// `-stackable0.0.0-dev` or `+dirty`. Without an expected version, every version matches.
pub fn matches_version(actual: &str, expected: Option<&str>) -> bool {
    let Some(expected) = expected else {
        return true;
    };
    let actual = percent_decode(actual);
    actual == expected
        || actual
            .strip_prefix(expected)
            .and_then(|rest| rest.chars().next())
            .is_some_and(|separator| "-+_".contains(separator))
}

/// Whether a license is a valid SPDX expression, `LicenseRef-`s included.
///
/// Deprecated IDs like `BSD-2-Clause-FreeBSD` are valid: they are still on the SPDX license list,
/// and mergebom keeps them on purpose when it has no replacement for them.
pub fn is_valid_license(expression: &str) -> bool {
    let mode = ::spdx::ParseMode {
        allow_deprecated: true,
        ..::spdx::ParseMode::STRICT
    };
    ::spdx::Expression::parse_mode(expression, mode).is_ok()
}

pub fn parse_timestamp(value: Option<&str>) -> Option<OffsetDateTime> {
    value.and_then(|value| OffsetDateTime::parse(value, &Rfc3339).ok())
}

/// A value from a document for a message: quoted, or `missing`.
pub fn describe(value: Option<&str>) -> String {
    value.map_or_else(|| "missing".to_owned(), |value| format!("{value:?}"))
}

/// A short, stable sample of a collection for a message.
pub fn examples<I, S>(items: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    const LIMIT: usize = 3;
    const MAX_LENGTH: usize = 90;

    let items: BTreeSet<String> = items
        .into_iter()
        .map(|item| {
            let item = item.as_ref();
            if item.chars().count() <= MAX_LENGTH {
                item.to_owned()
            } else {
                format!("{}...", item.chars().take(MAX_LENGTH).collect::<String>())
            }
        })
        .collect();
    let shown = items
        .iter()
        .take(LIMIT)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(", ");
    if items.len() <= LIMIT {
        shown
    } else {
        format!("{shown} and {} more", items.len() - LIMIT)
    }
}

pub fn share(part: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        part as f64 / total as f64
    }
}

pub fn percent(value: f64) -> String {
    format!("{:.1}%", value * 100.0)
}

#[cfg(test)]
mod tests;
