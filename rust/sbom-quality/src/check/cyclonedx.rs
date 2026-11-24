//! Checks of the CycloneDX SBOM.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::LazyLock,
};

use cyclonedx_bom::{
    models::bom::Bom,
    validation::{Validate, ValidationErrorsKind, ValidationResult},
};
use regex::Regex;
use serde_json::Value;
use time::OffsetDateTime;

use super::{
    ImageResult, ORGANIZATION, describe, examples, expected_image_purl, is_valid_license,
    matches_version, parse_timestamp, percent, share, substitute,
};
use crate::{
    expectations::Expectations,
    purl::{Purl, percent_decode},
    sbom::{Component, CycloneDx, Metadata},
};

const EMBEDDED_SBOM_CATALOGER: &str = "sbom-cataloger";
const RPM_DATABASE_CATALOGER: &str = "rpm-db-cataloger";
const IMAGE_LABEL_PREFIX: &str = "syft:image:labels:";

/// Rekor rejected the attestation of a 20 MB SBOM, so an SBOM that grows towards that is worth a
/// warning before the attestation step starts to fail.
const MAX_SBOM_BYTES: usize = 12 * 1024 * 1024;

/// RPMs that are legitimately installed in several versions.
const MULTI_VERSION_RPMS: [&str; 4] = ["gpg-pubkey", "kernel", "kernel-core", "kernel-modules"];

/// A license of a component in the paths of cyclonedx-bom's validation errors.
static LICENSE_PATH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:components\[(?P<component>\d+)\]|metadata\.component)\.licenses\[(?P<license>\d+)\]\.(?:expression|license\.license_identifier\.SpdxId)$",
    )
    .expect("static string must be a valid regular expression")
});

static INDEX_IN_PATH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[\d+\]").expect("static string must be a valid regular expression")
});

/// The only syft properties mergebom keeps on a component. Everything else is bookkeeping that
/// bloats the document.
static KEPT_SYFT_PROPERTY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^syft:(package:foundBy|metadata:virtualPath|location:\d+:path)$")
        .expect("static string must be a valid regular expression")
});

static LOCATION_PROPERTY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^syft:location:\d+:path$")
        .expect("static string must be a valid regular expression")
});

static INDEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r":\d+:").expect("static string must be a valid regular expression")
});

/// Validates the SBOM with the cyclonedx-bom crate, against its own specification version.
///
/// This catches what the other checks do not look at: invalid SPDX license IDs and expressions,
/// malformed purls and hashes, unknown enum values, duplicate bom-refs and dependencies that
/// point to no component. The crate reads the document strictly, so unlike the other checks it
/// cannot say anything about a document it fails to read.
pub fn check_schema(result: &mut ImageResult, json: &str, lenient: &CycloneDx) {
    let bom = match Bom::parse_from_json(json.as_bytes()) {
        Ok(bom) => bom,
        Err(error) => {
            result.error(
                "cdx-schema",
                format!("the SBOM is not valid CycloneDX: {error}"),
            );
            return;
        }
    };

    let mut errors = Vec::new();
    flatten_validation(bom.validate_version(bom.spec_version), "", &mut errors);
    // cyclonedx-bom also rejects deprecated SPDX IDs. The SPDX check accepts them, so this one
    // does as well, see `is_valid_license`.
    errors.retain(|(path, _)| !license_at(lenient, path).is_some_and(is_valid_license));

    // One finding per field, not per component: an invalid license of every RPM is one problem.
    let mut by_field = BTreeMap::<String, Vec<String>>::new();
    for (path, message) in errors {
        let field = INDEX_IN_PATH.replace_all(&path, "[N]").into_owned();
        by_field.entry(field).or_default().push(message);
    }
    for (field, messages) in by_field {
        result.error(
            "cdx-schema",
            format!(
                "{field}: {count} errors, e.g. {examples}",
                count = messages.len(),
                examples = examples(&messages),
            ),
        );
    }
}

/// The license that a validation error path like `components[3].licenses[0].expression` points
/// to.
fn license_at<'a>(bom: &'a CycloneDx, path: &str) -> Option<&'a str> {
    let captures = LICENSE_PATH.captures(path)?;
    let component = match captures.name("component") {
        Some(index) => bom.components.get(index.as_str().parse::<usize>().ok()?)?,
        None => bom.metadata.component.as_ref()?,
    };
    let license = component
        .licenses
        .get(captures.name("license")?.as_str().parse::<usize>().ok()?)?;
    license
        .get("expression")
        .or_else(|| license.pointer("/license/id"))?
        .as_str()
}

/// Turns the nested validation result into (path, message) pairs, like
/// `components[3].licenses[0].expression`.
fn flatten_validation(result: ValidationResult, path: &str, errors: &mut Vec<(String, String)>) {
    for (name, kind) in result.errors() {
        // Wrappers like `Components` nest their content under `inner`.
        let path = match name.as_str() {
            "" | "inner" => path.to_owned(),
            name if path.is_empty() => name.to_owned(),
            name => format!("{path}.{name}"),
        };
        let path_or_document = || {
            if path.is_empty() {
                "document".to_owned()
            } else {
                path.clone()
            }
        };
        match kind {
            ValidationErrorsKind::Struct(nested) => flatten_validation(nested, &path, errors),
            ValidationErrorsKind::List(items) => {
                for (index, nested) in items {
                    flatten_validation(nested, &format!("{path}[{index}]"), errors);
                }
            }
            ValidationErrorsKind::Field(list) | ValidationErrorsKind::Custom(list) => errors
                .extend(
                    list.into_iter()
                        .map(|error| (path_or_document(), error.message)),
                ),
            ValidationErrorsKind::Enum(error) => errors.push((path_or_document(), error.message)),
        }
    }
}

/// Runs every check on the CycloneDX SBOM. Returns the purls of its components.
pub fn check(
    result: &mut ImageResult,
    bom: &CycloneDx,
    size: usize,
    expectations: &Expectations,
) -> BTreeSet<String> {
    let missing_root = Component::default();
    let root = bom.metadata.component.as_ref().unwrap_or(&missing_root);

    check_format(result, bom, size, expectations);
    check_root(result, root, &expectations.registry);
    check_document_metadata(result, &bom.metadata, expectations);

    if bom.components.is_empty() {
        result.error("cdx-components", "the SBOM lists no components");
        return BTreeSet::new();
    }

    let purls = check_components(result, &bom.components, expectations);
    check_operating_system(result, &bom.components, expectations);
    check_dependency_graph(result, bom, root);
    check_component_counts(result, expectations);
    check_main_component(result, &purls, expectations);

    purls.into_iter().collect()
}

fn check_format(
    result: &mut ImageResult,
    bom: &CycloneDx,
    size: usize,
    expectations: &Expectations,
) {
    if bom.bom_format.as_deref() != Some("CycloneDX") {
        result.error(
            "cdx-format",
            format!(
                "bomFormat is {format}",
                format = describe(bom.bom_format.as_deref())
            ),
        );
    }

    let expected = &expectations.cyclonedx_spec_version;
    if bom.spec_version.as_ref() != Some(expected) {
        result.error(
            "cdx-format",
            format!(
                "specVersion is {version}, expected {expected}",
                version = bom.spec_version.as_deref().unwrap_or("missing")
            ),
        );
    }

    result.stats.bytes = Some(size);
    if size > MAX_SBOM_BYTES {
        result.warning(
            "cdx-size",
            format!(
                "the SBOM has {size:.1} MB, Rekor rejects attestations that get much larger \
                 (limit for this warning: {limit} MB)",
                size = size as f64 / 1024.0 / 1024.0,
                limit = MAX_SBOM_BYTES / 1024 / 1024,
            ),
        );
    }
}

/// The root component has to identify exactly the image the SBOM is attested to.
///
/// A wrong digest means the SBOM belongs to a different build, which is the one thing a consumer
/// cannot detect from the SBOM alone.
fn check_root(result: &mut ImageResult, root: &Component, registry: &str) {
    let target = result.target.clone();

    if root.component_type.as_deref() != Some("container") {
        result.error(
            "cdx-root-identity",
            format!(
                "root component type is {component_type}",
                component_type = describe(root.component_type.as_deref())
            ),
        );
    }
    if root.name.as_deref() != Some(target.image.as_str()) {
        result.error(
            "cdx-root-identity",
            format!(
                "root component name is {name}, expected {expected:?}",
                name = describe(root.name.as_deref()),
                expected = target.image
            ),
        );
    }
    if root.version.as_deref() != Some(target.canonical_tag().as_str()) {
        result.error(
            "cdx-root-identity",
            format!(
                "root component version is {version}, expected {expected:?}",
                version = describe(root.version.as_deref()),
                expected = target.canonical_tag()
            ),
        );
    }
    if let Some(digest) = result.digest.clone() {
        let purl = expected_image_purl(registry, &target, &digest);
        if root.purl.as_ref() != Some(&purl) {
            result.error(
                "cdx-root-identity",
                format!(
                    "root component purl is {actual}, expected {purl:?}",
                    actual = describe(root.purl.as_deref())
                ),
            );
        }
    }
    if root.bom_ref.is_none() {
        result.error("cdx-root-identity", "root component has no bom-ref");
    }

    for (role, entity) in [
        ("supplier", &root.supplier),
        ("manufacturer", &root.manufacturer),
    ] {
        let is_ours = entity.as_ref().is_some_and(|entity| {
            entity.name.as_deref() == Some(ORGANIZATION) && !entity.url.is_empty()
        });
        if !is_ours {
            result.error(
                "cdx-root-metadata",
                format!("root component {role} is not {ORGANIZATION} with a URL"),
            );
        }
    }
    if root.description.as_deref().is_none_or(str::is_empty) {
        result.error("cdx-root-metadata", "root component has no description");
    }
    if root.licenses.is_empty() {
        result.error("cdx-root-metadata", "root component has no license");
    }
}

fn check_document_metadata(
    result: &mut ImageResult,
    metadata: &Metadata,
    expectations: &Expectations,
) {
    if parse_timestamp(metadata.timestamp.as_deref()).is_none() {
        result.error(
            "cdx-document-metadata",
            "metadata.timestamp is missing or invalid",
        );
    }

    if !metadata
        .authors
        .iter()
        .any(|author| author.name.as_deref() == Some(ORGANIZATION))
    {
        result.error(
            "cdx-document-metadata",
            format!("metadata.authors does not name {ORGANIZATION}"),
        );
    }

    if !metadata
        .lifecycles
        .iter()
        .any(|lifecycle| lifecycle.phase.as_deref() == Some("post-build"))
    {
        result.error(
            "cdx-document-metadata",
            "metadata.lifecycles does not say post-build",
        );
    }

    let tools: BTreeMap<&str, &str> = metadata
        .tools
        .get("components")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|tool| Some((tool.get("name")?.as_str()?, tool.get("version")?.as_str()?)))
        .collect();
    for tool in ["syft", "mergebom"] {
        if tools.get(tool).is_none_or(|version| version.is_empty()) {
            result.error(
                "cdx-document-metadata",
                format!("metadata.tools does not list {tool} with a version"),
            );
        }
    }

    // The image labels end up in the SBOM, so the age of the image needs no registry access.
    let created = metadata
        .properties
        .iter()
        .find(|property| {
            property.name.as_deref()
                == Some(&format!(
                    "{IMAGE_LABEL_PREFIX}org.opencontainers.image.created"
                ))
        })
        .and_then(|property| parse_timestamp(property.value.as_deref()));
    let max_age = expectations.max_image_age_days;
    if let Some(created) = created
        && (OffsetDateTime::now_utc() - created).whole_days() > max_age
    {
        result.warning(
            "image-age",
            format!(
                "the image was built on {date}, more than {max_age} days ago. The scheduled \
                 build is probably failing",
                date = created.date(),
            ),
        );
    }
}

/// Every component needs what a scanner or a human needs to identify it. Returns the purls.
fn check_components(
    result: &mut ImageResult,
    components: &[Component],
    expectations: &Expectations,
) -> Vec<String> {
    let unnamed = components
        .iter()
        .filter(|component| component.name.as_deref().is_none_or(str::is_empty))
        .count();
    if unnamed > 0 {
        result.error(
            "cdx-component-identity",
            format!("{unnamed} components have no name"),
        );
    }

    // CycloneDX allows components without a bom-ref, but then nothing can depend on them.
    // Duplicate bom-refs are reported by the schema check.
    let missing_bom_refs = components
        .iter()
        .filter(|component| component.bom_ref.is_none())
        .count();
    if missing_bom_refs > 0 {
        result.error(
            "cdx-component-identity",
            format!("{missing_bom_refs} components have no bom-ref"),
        );
    }

    let files = components
        .iter()
        .filter(|component| component.component_type.as_deref() == Some("file"))
        .count();
    if files > 0 {
        result.error(
            "cdx-file-components",
            format!(
                "{files} components of type file. The file catalogers of syft are meant to be \
                 off, they multiply the size of the SBOM"
            ),
        );
    }

    let packages: Vec<&Component> = components
        .iter()
        .filter(|component| component.component_type.as_deref() != Some("operating-system"))
        .collect();
    let name = |component: &&Component| component.name.clone().unwrap_or_default();

    // A few components really have no package identity, like the Windows launcher stubs that
    // pip ships. More than a few means a cataloger lost it.
    let without_purl: Vec<&Component> = packages
        .iter()
        .copied()
        .filter(|component| component.purl.as_deref().is_none_or(str::is_empty))
        .collect();
    let allowed = expectations.max_without_purl;
    if share(without_purl.len(), packages.len()) > allowed {
        result.error(
            "cdx-purl",
            format!(
                "{count} of {total} components have no purl (allowed: {allowed}), e.g. \
                 {examples}",
                count = without_purl.len(),
                total = packages.len(),
                allowed = percent(allowed),
                examples = examples(without_purl.iter().map(name)),
            ),
        );
    }

    let purls: Vec<&str> = packages
        .iter()
        .filter_map(|component| component.purl.as_deref())
        .filter(|purl| !purl.is_empty())
        .collect();
    let without_version: Vec<&Component> = packages
        .iter()
        .copied()
        .filter(|component| {
            component
                .version
                .as_deref()
                .is_none_or(|version| version.is_empty() || version == "UNKNOWN")
        })
        .collect();
    let allowed = expectations.max_without_version;
    if share(without_version.len(), packages.len()) > allowed {
        result.error(
            "cdx-component-version",
            format!(
                "{count} of {total} components have no version (allowed: {allowed}), e.g. \
                 {examples}",
                count = without_version.len(),
                total = packages.len(),
                allowed = percent(allowed),
                examples = examples(without_version.iter().map(name)),
            ),
        );
    }

    // mergebom merges components with the same purl, so a duplicate means it did not run or
    // regressed.
    let mut purl_counts = BTreeMap::<&str, usize>::new();
    for purl in &purls {
        *purl_counts.entry(purl).or_default() += 1;
    }
    let duplicate_purls = duplicates(&purl_counts);
    if !duplicate_purls.is_empty() {
        result.error(
            "cdx-duplicate-purls",
            format!(
                "{count} purls are listed more than once, e.g. {examples}",
                count = duplicate_purls.len(),
                examples = examples(duplicate_purls),
            ),
        );
    }

    check_rpm_versions(result, &packages);

    // The number of components per syft property, with the index in its name replaced.
    let bloat = components
        .iter()
        .flat_map(|component| &component.properties)
        .filter_map(|property| property.name.as_deref())
        .filter(|name| name.starts_with("syft:") && !KEPT_SYFT_PROPERTY.is_match(name))
        .fold(BTreeMap::<String, usize>::new(), |mut bloat, name| {
            *bloat
                .entry(INDEX.replace_all(name, ":N:").into_owned())
                .or_default() += 1;
            bloat
        });
    if !bloat.is_empty() {
        result.error(
            "cdx-syft-properties",
            format!(
                "{count} syft bookkeeping properties were not removed, e.g. {examples}",
                count = bloat.values().sum::<usize>(),
                examples = examples(bloat.keys()),
            ),
        );
    }

    let with_license = packages
        .iter()
        .filter(|component| !component.licenses.is_empty())
        .count();
    result.stats.license_coverage = Some(share(with_license, packages.len()));
    result.stats.components = component_counts(components);
    result.stats.embedded_sboms = embedded_sboms(components);

    purls.into_iter().map(ToOwned::to_owned).collect()
}

/// Scanning all layers instead of the squashed filesystem reports every RPM that a later layer
/// updated twice, and the outdated one produces false vulnerability findings.
///
/// Only the RPM database counts: Python wheels vendor libraries of other distributions and say so
/// in an SBOM of their own, which legitimately adds a second version of an RPM.
fn check_rpm_versions(result: &mut ImageResult, packages: &[&Component]) {
    let mut rpm_versions = BTreeMap::<(String, Option<String>), BTreeSet<Option<String>>>::new();
    for component in packages {
        if component.found_by() != Some(RPM_DATABASE_CATALOGER) {
            continue;
        }
        let Some(purl) = component.purl.as_deref().and_then(Purl::parse) else {
            continue;
        };
        if purl.package_type == "rpm" {
            rpm_versions
                .entry((
                    purl.short_name().to_owned(),
                    purl.qualifiers.get("arch").cloned(),
                ))
                .or_default()
                .insert(purl.version);
        }
    }

    let superseded: BTreeSet<&str> = rpm_versions
        .iter()
        .filter(|((name, _), versions)| {
            versions.len() > 1 && !MULTI_VERSION_RPMS.contains(&name.as_str())
        })
        .map(|((name, _), _)| name.as_str())
        .collect();
    if !superseded.is_empty() {
        result.error(
            "cdx-rpm-versions",
            format!(
                "{count} RPMs are listed in more than one version, e.g. {examples}. Is syft \
                 scanning all layers?",
                count = superseded.len(),
                examples = examples(&superseded),
            ),
        );
    }
}

/// Returns the keys of a map where the value (signifying a count) is greater than 1.
fn duplicates<'a>(map: &BTreeMap<&'a str, usize>) -> Vec<&'a str> {
    map.iter()
        .filter(|(_, value)| **value > 1)
        .map(|(key, _)| *key)
        .collect()
}

/// Component counts: `total`, one per purl type (`maven`), and one per purl type for the
/// components that come from a build-time SBOM embedded in the image (`embedded-maven`).
///
/// The embedded ones are counted separately because syft finds most jars and Python packages on
/// its own as well. Without the build-time SBOM the plain count hardly moves, although the purls
/// get worse and everything that is shaded or bundled disappears.
fn component_counts(components: &[Component]) -> BTreeMap<String, u64> {
    let mut counts = BTreeMap::<String, u64>::new();
    for component in components {
        *counts.entry("total".to_owned()).or_default() += 1;
        let Some(purl) = component.purl.as_deref().and_then(Purl::parse) else {
            continue;
        };
        if component.found_by() == Some(EMBEDDED_SBOM_CATALOGER) {
            *counts
                .entry(format!("embedded-{kind}", kind = purl.package_type))
                .or_default() += 1;
        }
        *counts.entry(purl.package_type).or_default() += 1;
    }
    counts
}

/// The build-time SBOMs in the image, as path and number of components taken from it.
fn embedded_sboms(components: &[Component]) -> BTreeMap<String, u64> {
    let mut paths = BTreeMap::<String, u64>::new();
    for component in components {
        if component.found_by() != Some(EMBEDDED_SBOM_CATALOGER) {
            continue;
        }
        for property in &component.properties {
            if let (Some(name), Some(path)) = (property.name.as_deref(), property.value.as_deref())
                && LOCATION_PROPERTY.is_match(name)
            {
                *paths.entry(path.to_owned()).or_default() += 1;
            }
        }
    }
    paths
}

/// Scanners need the distribution to match RPMs against the right advisories.
fn check_operating_system(
    result: &mut ImageResult,
    components: &[Component],
    expectations: &Expectations,
) {
    let expected = &expectations.operating_system;
    let systems: Vec<&Component> = components
        .iter()
        .filter(|component| component.component_type.as_deref() == Some("operating-system"))
        .collect();
    if systems.len() != 1 {
        result.error(
            "cdx-operating-system",
            format!(
                "{count} operating-system components, expected 1",
                count = systems.len()
            ),
        );
    }
    for system in systems {
        // For RHEL and UBI, mergebom renames syft's "rhel" to the purl namespace "redhat".
        if system.name.as_ref() != Some(expected)
            || system.version.as_deref().is_none_or(str::is_empty)
        {
            result.error(
                "cdx-operating-system",
                format!(
                    "operating system is {name} {version}, expected {expected:?} with a version",
                    name = describe(system.name.as_deref()),
                    version = describe(system.version.as_deref()),
                ),
            );
        }
    }

    let rpms: Vec<&str> = components
        .iter()
        .filter(|component| component.found_by() == Some(RPM_DATABASE_CATALOGER))
        .map(|component| component.purl.as_deref().unwrap_or_default())
        .collect();
    if rpms.is_empty() {
        result.error(
            "cdx-operating-system",
            "no components from the RPM database",
        );
    }
    let namespace = format!("pkg:rpm/{expected}/");
    let wrong: Vec<&str> = rpms
        .iter()
        .copied()
        .filter(|purl| {
            !purl.starts_with(&namespace)
                || Purl::parse(purl).is_none_or(|purl| !purl.qualifiers.contains_key("distro"))
        })
        .collect();
    if !wrong.is_empty() {
        result.error(
            "cdx-operating-system",
            format!(
                "{count} of {total} installed RPMs lack the purl namespace {expected} or the \
                 distro qualifier, e.g. {examples}",
                count = wrong.len(),
                total = rpms.len(),
                examples = examples(wrong),
            ),
        );
    }
}

/// Every component has to be reachable from the image, otherwise a consumer that walks the graph
/// (and the SPDX document, which is built from it) loses components.
fn check_dependency_graph(result: &mut ImageResult, bom: &CycloneDx, root: &Component) {
    if bom.dependencies.is_empty() {
        result.error("cdx-dependency-graph", "the SBOM has no dependencies");
        return;
    }

    let known: BTreeSet<&str> = bom
        .components
        .iter()
        .filter_map(|component| component.bom_ref.as_deref())
        .collect();
    let root_bom_ref = root.bom_ref.as_deref();

    let mut graph = BTreeMap::<&str, BTreeSet<&str>>::new();
    for dependency in &bom.dependencies {
        if let Some(reference) = dependency.reference.as_deref() {
            graph
                .entry(reference)
                .or_default()
                .extend(dependency.depends_on.iter().map(String::as_str));
        }
    }

    // Dependencies that point to no component are reported by the schema check.
    let Some((root_bom_ref, root_dependencies)) = root_bom_ref
        .and_then(|root_bom_ref| Some((root_bom_ref, graph.get(root_bom_ref)?)))
        .filter(|(_, dependencies)| !dependencies.is_empty())
    else {
        result.error(
            "cdx-dependency-graph",
            "the root component has no dependencies",
        );
        return;
    };
    result.stats.direct_dependencies = Some(root_dependencies.len());

    let mut reachable = BTreeSet::new();
    let mut pending = vec![root_bom_ref];
    while let Some(reference) = pending.pop() {
        if reachable.insert(reference) {
            pending.extend(graph.get(reference).into_iter().flatten());
        }
    }

    let orphans: BTreeSet<&str> = known.difference(&reachable).copied().collect();
    if !orphans.is_empty() {
        result.error(
            "cdx-dependency-graph",
            format!(
                "{count} of {total} components are not reachable from the root, e.g. {examples}",
                count = orphans.len(),
                total = known.len(),
                examples = examples(&orphans),
            ),
        );
    }
}

/// Compares the component counts with the floors in the expectations.
///
/// This is the check for everything that the invariants cannot see: an SBOM without the Python
/// packages of Airflow is still a perfectly valid SBOM.
fn check_component_counts(result: &mut ImageResult, expectations: &Expectations) {
    let mut findings = Vec::new();

    for (kind, minimum) in &expectations.min_components {
        let actual = result
            .stats
            .components
            .get(kind)
            .copied()
            .unwrap_or_default();
        if actual < *minimum {
            let what = if kind == "total" {
                "components in total".to_owned()
            } else if let Some(package_type) = kind.strip_prefix("embedded-") {
                format!("{package_type} components from build-time SBOMs")
            } else {
                format!("{kind} components")
            };
            findings.push((
                "component-count",
                format!("{actual} {what}, expected at least {minimum}"),
            ));
        }
    }

    if let (Some(minimum), Some(actual)) = (
        expectations.min_license_coverage,
        result.stats.license_coverage,
    ) && actual < minimum
    {
        findings.push((
            "license-coverage",
            format!(
                "{actual} of the components have a license, expected at least {minimum}",
                actual = percent(actual),
                minimum = percent(minimum)
            ),
        ));
    }

    // The floors above have a margin and several build-time SBOMs feed the same count, so a small
    // one can get lost unnoticed. This cannot happen to the ones listed by path.
    for pattern in &expectations.embedded_sboms {
        let path = substitute(pattern, &result.target, &expectations.release);
        let found = glob::Pattern::new(&path).is_ok_and(|pattern| {
            result
                .stats
                .embedded_sboms
                .keys()
                .any(|found| pattern.matches(found))
        });
        if !found {
            findings.push((
                "embedded-sbom",
                format!("no component from the build-time SBOM {path}"),
            ));
        }
    }

    for (check, message) in findings {
        result.error(check, message);
    }
}

/// The product itself has to be in the SBOM, in the version the image is tagged with.
fn check_main_component(result: &mut ImageResult, purls: &[String], expectations: &Expectations) {
    let without_qualifiers: Vec<&str> = purls
        .iter()
        .map(|purl| purl.split_once('?').map_or(purl.as_str(), |(purl, _)| purl))
        .collect();

    for pattern in &expectations.main_components {
        let expected = substitute(pattern, &result.target, &expectations.release);
        let (name, version) = match expected.rsplit_once('@') {
            Some((name, version)) => (name, Some(version).filter(|version| !version.is_empty())),
            None => (expected.as_str(), None),
        };

        let found: Vec<&str> = without_qualifiers
            .iter()
            .filter_map(|purl| purl.rsplit_once('@'))
            .filter(|(purl_name, _)| *purl_name == name)
            .map(|(_, purl_version)| purl_version)
            .collect();
        if !found.iter().any(|actual| matches_version(actual, version)) {
            let other = if found.is_empty() {
                "not listed at all".to_owned()
            } else {
                examples(found.iter().map(|version| percent_decode(version)))
            };
            result.error(
                "main-component",
                format!(
                    "no component {name} in version {version} (found: {other})",
                    version = version.unwrap_or("any"),
                ),
            );
        }
    }
}
