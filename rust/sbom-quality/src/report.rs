//! Prints the results: as text for humans, as JSON for machines, or as floors to paste into the
//! expectations.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    sync::LazyLock,
};

use regex::Regex;
use serde::Serialize;

use crate::{
    check::{ImageResult, Severity, Stats},
    expectations::{ExpectationsFile, Target},
};

/// The maximum number of build-time SBOMs that `--suggest-floors` suggests for `embedded-sboms`.
/// The ones with the most components are listed first.
///
/// Some images have a build-time SBOM per plugin (NiFi has one per NAR, around a hundred), and
/// listing every one of them would bloat the expectations. The ones that are cut off are still
/// covered in aggregate by the `embedded-<type>` floors in `min-components`.
//
// Raise it when the suggested list of a product is regularly cut off and build-time SBOMs that
// should be checked by path have to be added to the expectations by hand. Lower it when the review
// regularly drops most of the suggested paths again.
const MAX_LISTED_SBOMS: usize = 12;

static NUMBER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d+").expect("static string must be a valid regular expression"));

/// The findings as text, grouped by check and by kind of message, because a regression in the
/// pipeline hits every image at once and should read as one finding, not as two hundred.
pub struct TextReport<'a> {
    results: &'a [ImageResult],
    problems: &'a [String],
    verbose: bool,
}

impl<'a> TextReport<'a> {
    pub fn new(results: &'a [ImageResult], problems: &'a [String], verbose: bool) -> Self {
        Self {
            results,
            problems,
            verbose,
        }
    }
}

impl fmt::Display for TextReport<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (results, problems, verbose) = (self.results, self.problems, self.verbose);
        type Found<'a> = Vec<(&'a Target, &'a str)>;
        let mut grouped = BTreeMap::<(Severity, &str), BTreeMap<String, Found<'_>>>::new();
        for result in results {
            for finding in &result.findings {
                // Messages that only differ in their numbers are the same kind of finding.
                let kind = if verbose {
                    finding.message.clone()
                } else {
                    NUMBER.replace_all(&finding.message, "N").into_owned()
                };
                grouped
                    .entry((finding.severity, finding.check))
                    .or_default()
                    .entry(kind)
                    .or_default()
                    .push((&result.target, &finding.message));
            }
        }

        for problem in problems {
            writeln!(f, "ERROR   expectations\n        {problem}\n")?;
        }

        for ((severity, check), kinds) in &grouped {
            let affected: BTreeSet<&Target> = kinds
                .values()
                .flatten()
                .map(|(target, _)| *target)
                .collect();
            let label = match severity {
                Severity::Error => "ERROR  ",
                Severity::Warning => "WARNING",
            };
            writeln!(
                f,
                "{label} {check}  ({affected} of {total} images)",
                affected = affected.len(),
                total = results.len()
            )?;

            let mut shown: Vec<&Found<'_>> = kinds.values().collect();
            shown.sort_by_key(|found| {
                (
                    std::cmp::Reverse(found.len()),
                    found.first().map(|(_, message)| *message),
                )
            });
            let limit = if verbose { shown.len() } else { 4 };
            for found in shown.iter().take(limit) {
                let names: Vec<String> =
                    found.iter().map(|(target, _)| target.to_string()).collect();
                let mut location = if verbose {
                    names.join(", ")
                } else {
                    names.iter().take(2).cloned().collect::<Vec<_>>().join(", ")
                };
                if !verbose && names.len() > 2 {
                    location.push_str(&format!(" and {more} more", more = names.len() - 2));
                }
                let message = found
                    .first()
                    .map(|(_, message)| *message)
                    .unwrap_or_default();
                writeln!(f, "        {message}\n          {location}")?;
            }
            if shown.len() > limit {
                writeln!(
                    f,
                    "        ... and {more} more kinds, see --verbose",
                    more = shown.len() - limit
                )?;
            }
            writeln!(f)?;
        }

        let count = |severity| {
            results
                .iter()
                .flat_map(|result| &result.findings)
                .filter(|finding| finding.severity == severity)
                .count()
        };
        let clean = results
            .iter()
            .filter(|result| result.findings.is_empty())
            .count();
        let ignored: usize = results.iter().map(|result| result.ignored).sum();
        let mut summary = format!(
            "{total} images checked: {clean} clean, {errors} errors, {warnings} warnings",
            total = results.len(),
            errors = count(Severity::Error) + problems.len(),
            warnings = count(Severity::Warning),
        );
        if ignored > 0 {
            summary.push_str(&format!(", {ignored} findings ignored"));
        }
        writeln!(f, "{summary}")
    }
}

#[derive(Serialize)]
struct JsonOutput<'a> {
    problems: &'a [String],
    images: Vec<JsonImage<'a>>,
}

#[derive(Serialize)]
struct JsonImage<'a> {
    image: &'a str,
    version: Option<&'a str>,
    tag: String,
    architecture: &'a str,
    digest: Option<&'a str>,
    stats: &'a Stats,
    ignored: usize,
    findings: &'a [crate::check::Finding],
}

/// The findings and stats of every image as JSON.
pub fn to_json(results: &[ImageResult], problems: &[String]) -> Result<String, serde_json::Error> {
    let output = JsonOutput {
        problems,
        images: results
            .iter()
            .map(|result| JsonImage {
                image: &result.target.image,
                version: result.target.version.as_deref(),
                tag: result.target.canonical_tag(),
                architecture: &result.target.architecture,
                digest: result.digest.as_deref(),
                stats: &result.stats,
                ignored: result.ignored,
                findings: &result.findings,
            })
            .collect(),
    };
    serde_json::to_string_pretty(&output)
}

/// Rounds down to two significant digits, so the floors do not pretend to be exact.
pub fn round_down(value: f64) -> u64 {
    let whole = value.max(0.0) as u64;
    if whole < 100 {
        return whole;
    }
    let magnitude = 10_u64.pow(whole.to_string().len() as u32 - 2);
    whole / magnitude * magnitude
}

/// A path of an image as a pattern for `embedded-sboms`.
fn path_pattern(path: &str, target: &Target, release: &str) -> String {
    let path = match target.version.as_deref() {
        // Short versions like the 11 of java-base would also replace parts of other numbers.
        Some(version) if version.len() > 2 => path.replace(version, "{version}"),
        Some(_) => path.to_owned(),
        None => path.replace(&target.image, "{image}"),
    };
    path.replace(release, "{release}")
}

/// Floors for the expectations from what is published right now: the smallest count over all
/// versions and architectures of a product, minus a margin.
///
/// Review them before pasting. A product whose versions differ a lot is better served by
/// `versions."<version>".min-components`.
pub struct SuggestedFloors<'a> {
    expectations: &'a ExpectationsFile,
    results: &'a [ImageResult],
    margin: f64,
}

impl<'a> SuggestedFloors<'a> {
    pub fn new(
        expectations: &'a ExpectationsFile,
        results: &'a [ImageResult],
        margin: f64,
    ) -> Self {
        Self {
            expectations,
            results,
            margin,
        }
    }
}

impl fmt::Display for SuggestedFloors<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (expectations, results, margin) = (self.expectations, self.results, self.margin);

        let mut by_group = BTreeMap::<Option<&str>, Vec<&ImageResult>>::new();
        for result in results {
            if result.stats.components.is_empty() {
                continue;
            }
            let image = result.target.image.as_str();
            let group = expectations.products.contains_key(image).then_some(image);
            by_group.entry(group).or_default().push(result);
        }

        for (group, members) in &by_group {
            let table = group.map_or_else(
                || "operators".to_owned(),
                |product| format!("products.{product}"),
            );

            let coverage = members
                .iter()
                .filter_map(|member| member.stats.license_coverage)
                .fold(1.0_f64, f64::min);
            writeln!(f, "[{table}]")?;
            writeln!(
                f,
                "min-license-coverage = {minimum}",
                minimum = ((coverage - 0.05) * 100.0).floor().max(0.0) / 100.0
            )?;

            // The build-time SBOMs that every version and architecture has, biggest first. An image
            // with a hundred of them (NiFi has one per NAR) is not listed completely.
            let patterns: Vec<BTreeMap<String, u64>> = members
                .iter()
                .map(|member| {
                    member
                        .stats
                        .embedded_sboms
                        .iter()
                        .map(|(path, count)| {
                            (
                                path_pattern(path, &member.target, &expectations.release),
                                *count,
                            )
                        })
                        .collect()
                })
                .collect();
            let mut common: Vec<(&String, u64)> = patterns
                .first()
                .into_iter()
                .flat_map(BTreeMap::keys)
                .filter(|path| patterns.iter().all(|pattern| pattern.contains_key(*path)))
                .map(|path| {
                    let smallest = patterns
                        .iter()
                        .filter_map(|pattern| pattern.get(path))
                        .min()
                        .copied()
                        .unwrap_or_default();
                    (path, smallest)
                })
                .collect();
            common.sort_by_key(|(path, smallest)| (std::cmp::Reverse(*smallest), *path));
            let mut listed: Vec<&String> = common
                .into_iter()
                .take(MAX_LISTED_SBOMS)
                .map(|(path, _)| path)
                .collect();
            listed.sort();
            if !listed.is_empty() {
                writeln!(f, "embedded-sboms = [")?;
                for path in listed {
                    writeln!(f, "  \"{path}\",")?;
                }
                writeln!(f, "]")?;
            }

            writeln!(f, "\n[{table}.min-components]")?;
            let mut kinds: Vec<&String> = members
                .first()
                .into_iter()
                .flat_map(|member| member.stats.components.keys())
                .filter(|kind| {
                    members
                        .iter()
                        .all(|member| member.stats.components.contains_key(*kind))
                })
                .collect();
            kinds.sort_by_key(|kind| (*kind != "total", kind.starts_with("embedded-"), *kind));
            for kind in kinds {
                let counts = members
                    .iter()
                    .filter_map(|member| member.stats.components.get(kind).copied());
                let smallest = counts.clone().min().unwrap_or_default();
                let largest = counts.max().unwrap_or_default();
                let floor = round_down(smallest as f64 * (1.0 - margin));
                // A handful of components of some type is noise, not something to guard.
                if floor >= 10 {
                    writeln!(f, "{kind} = {floor}  # observed: {smallest} to {largest}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(7.9, 7)]
    #[case(99.0, 99)]
    #[case(112.0, 110)]
    #[case(1149.6, 1100)]
    #[case(-3.0, 0)]
    fn round_floors_down(#[case] value: f64, #[case] floor: u64) {
        assert_eq!(round_down(value), floor);
    }

    #[rstest]
    #[case::product(
        "/stackable/kafka_2.13-4.3.1-stackable0.0.0-dev/kafka.cdx.json",
        Some("4.3.1"),
        "/stackable/kafka_2.13-{version}-stackable{release}/kafka.cdx.json"
    )]
    #[case::short_version(
        "/stackable/jdk-11/sbom.cdx.json",
        Some("11"),
        "/stackable/jdk-11/sbom.cdx.json"
    )]
    #[case::operator(
        "/usr/local/bin/stackable-kafka-operator_bin.cdx.xml",
        None,
        "/usr/local/bin/stackable-{image}_bin.cdx.xml"
    )]
    fn turn_paths_into_patterns(
        #[case] path: &str,
        #[case] version: Option<&str>,
        #[case] pattern: &str,
    ) {
        let target = Target {
            image: if version.is_some() {
                "kafka"
            } else {
                "kafka-operator"
            }
            .to_owned(),
            version: version.map(ToOwned::to_owned),
            tag: "tag".to_owned(),
            architecture: "amd64".to_owned(),
        };
        assert_eq!(path_pattern(path, &target, "0.0.0-dev"), pattern);
    }
}
