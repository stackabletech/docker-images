//! Checks of the SPDX SBOM.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::LazyLock,
};

use regex::Regex;

use super::{
    ImageResult, ORGANIZATION, describe, examples, expected_image_purl, is_valid_license,
    parse_timestamp, share,
};
use crate::sbom::{Package, Relationship, Spdx};

const DOCUMENT_ID: &str = "SPDXRef-DOCUMENT";

/// Share of third-party packages that may name us as supplier. `syft convert` once set us as the
/// supplier of every package.
const MAX_SHARE_STACKABLE_SUPPLIER: f64 = 0.05;

static SPDX_ID: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^SPDXRef-[A-Za-z0-9.-]+$")
        .expect("static string must be a valid regular expression")
});

static LICENSE_REF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"LicenseRef-[A-Za-z0-9.-]+")
        .expect("static string must be a valid regular expression")
});

static OUR_CREATOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"^Organization: {ORGANIZATION} \(.+@.+\)"))
        .expect("static string must be a valid regular expression")
});

/// Runs every check on the SPDX SBOM. Returns the purls of its packages, without the image.
pub fn check(result: &mut ImageResult, document: &Spdx, registry: &str) -> BTreeSet<String> {
    check_format(result, document);

    if document.packages.is_empty() {
        result.error("spdx-packages", "the document lists no packages");
        return BTreeSet::new();
    }
    check_packages(result, &document.packages);

    let by_id: BTreeMap<&str, &Package> = document
        .packages
        .iter()
        .filter_map(|package| Some((package.spdx_id.as_deref()?, package)))
        .collect();

    let described: BTreeSet<&str> = document
        .relationships
        .iter()
        .filter(|relationship| {
            relationship.relationship_type.as_deref() == Some("DESCRIBES")
                && relationship.spdx_element_id.as_deref() == Some(DOCUMENT_ID)
        })
        .filter_map(|relationship| relationship.related_spdx_element.as_deref())
        .chain(document.document_describes.iter().map(String::as_str))
        .collect();
    let root = match described.first() {
        Some(root_id) if described.len() == 1 => by_id.get(root_id).copied(),
        _ => None,
    };
    let (Some(root), Some(root_id)) = (root, described.first().copied()) else {
        result.error(
            "spdx-root",
            format!(
                "the document describes {} packages, expected 1",
                described.len()
            ),
        );
        return BTreeSet::new();
    };

    let others: Vec<&Package> = document
        .packages
        .iter()
        .filter(|package| package.spdx_id.as_deref() != Some(root_id))
        .collect();

    check_root(result, root, registry);
    check_suppliers(result, &others);
    check_license_refs(result, document);
    check_license_expressions(result, &document.packages);
    check_relationships(result, &document.relationships, &by_id, root_id);

    others
        .iter()
        .flat_map(|package| package.purls())
        .map(ToOwned::to_owned)
        .collect()
}

fn check_format(result: &mut ImageResult, document: &Spdx) {
    if document.spdx_version.as_deref() != Some("SPDX-2.3") {
        result.error(
            "spdx-format",
            format!(
                "spdxVersion is {}",
                describe(document.spdx_version.as_deref())
            ),
        );
    }
    if document.data_license.as_deref() != Some("CC0-1.0") {
        result.error(
            "spdx-format",
            format!(
                "dataLicense is {}",
                describe(document.data_license.as_deref())
            ),
        );
    }
    if document.spdx_id.as_deref() != Some(DOCUMENT_ID) {
        result.error(
            "spdx-format",
            format!("SPDXID is {}", describe(document.spdx_id.as_deref())),
        );
    }
    // The namespace itself is not printed, it is different in every document.
    if !document
        .document_namespace
        .as_deref()
        .is_some_and(|namespace| namespace.starts_with("https://stackable.tech/"))
    {
        result.error(
            "spdx-format",
            "documentNamespace is not below https://stackable.tech/",
        );
    }

    let creation = &document.creation_info;
    if parse_timestamp(creation.created.as_deref()).is_none() {
        result.error(
            "spdx-creation-info",
            "creationInfo.created is missing or invalid",
        );
    }
    if !creation
        .creators
        .iter()
        .any(|creator| OUR_CREATOR.is_match(creator))
    {
        result.error(
            "spdx-creation-info",
            format!("creationInfo.creators does not name {ORGANIZATION} with an email address"),
        );
    }
    for tool in ["syft", "mergebom"] {
        if !creation
            .creators
            .iter()
            .any(|creator| creator.starts_with(&format!("Tool: {tool}-")))
        {
            result.error(
                "spdx-creation-info",
                format!("creationInfo.creators does not list {tool}"),
            );
        }
    }
}

fn check_packages(result: &mut ImageResult, packages: &[Package]) {
    let mut ids = BTreeMap::<&str, usize>::new();
    for package in packages {
        *ids.entry(package.spdx_id.as_deref().unwrap_or("missing"))
            .or_default() += 1;
    }

    let invalid: Vec<&str> = ids
        .keys()
        .copied()
        .filter(|spdx_id| !SPDX_ID.is_match(spdx_id))
        .collect();
    if !invalid.is_empty() {
        result.error(
            "spdx-packages",
            format!(
                "{count} invalid SPDXIDs, e.g. {examples}",
                count = invalid.len(),
                examples = examples(&invalid),
            ),
        );
    }

    let duplicate: Vec<&str> = ids
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|(spdx_id, _)| *spdx_id)
        .collect();
    if !duplicate.is_empty() {
        result.error(
            "spdx-packages",
            format!(
                "{count} SPDXIDs are used more than once, e.g. {examples}",
                count = duplicate.len(),
                examples = examples(&duplicate),
            ),
        );
    }

    let incomplete = packages
        .iter()
        .filter(|package| {
            package.name.as_deref().is_none_or(str::is_empty)
                || package
                    .download_location
                    .as_deref()
                    .is_none_or(str::is_empty)
        })
        .count();
    if incomplete > 0 {
        result.error(
            "spdx-packages",
            format!("{incomplete} packages lack a name or a downloadLocation"),
        );
    }
}

fn check_root(result: &mut ImageResult, root: &Package, registry: &str) {
    let target = result.target.clone();

    if root.primary_package_purpose.as_deref() != Some("CONTAINER") {
        result.error(
            "spdx-root",
            format!(
                "primaryPackagePurpose of the image is {}",
                describe(root.primary_package_purpose.as_deref())
            ),
        );
    }
    if root.name.as_deref() != Some(target.image.as_str())
        || root.version_info.as_deref() != Some(target.canonical_tag().as_str())
    {
        result.error(
            "spdx-root",
            format!(
                "the image package is {name} {version}, expected {image:?} {tag:?}",
                name = describe(root.name.as_deref()),
                version = describe(root.version_info.as_deref()),
                image = target.image,
                tag = target.canonical_tag(),
            ),
        );
    }
    if let Some(digest) = result.digest.clone() {
        let purl = expected_image_purl(registry, &target, &digest);
        if root.purls() != [purl.as_str()] {
            result.error(
                "spdx-root",
                format!(
                    "the image package purl is {actual:?}, expected {purl:?}",
                    actual = root.purls()
                ),
            );
        }
    }
    let supplier = format!("Organization: {ORGANIZATION}");
    if root.supplier.as_ref() != Some(&supplier) {
        result.error(
            "spdx-root",
            format!(
                "supplier of the image is {}",
                describe(root.supplier.as_deref())
            ),
        );
    }
    if root
        .license_declared
        .as_deref()
        .is_none_or(|license| ["", "NOASSERTION", "NONE"].contains(&license))
    {
        result.error("spdx-root", "the image package declares no license");
    }
}

/// We are the supplier of the image, not of the packages in it.
fn check_suppliers(result: &mut ImageResult, others: &[&Package]) {
    let ours = others
        .iter()
        .filter(|package| {
            package
                .supplier
                .as_deref()
                .is_some_and(|supplier| supplier.contains(ORGANIZATION))
        })
        .count();
    if share(ours, others.len()) > MAX_SHARE_STACKABLE_SUPPLIER {
        result.error(
            "spdx-supplier",
            format!(
                "{ours} of {total} third-party packages name {ORGANIZATION} as supplier",
                total = others.len()
            ),
        );
    }
}

/// A LicenseRef that is not declared makes the document invalid for strict parsers.
fn check_license_refs(result: &mut ImageResult, document: &Spdx) {
    let mut declared = BTreeMap::<&str, usize>::new();
    for license in &document.has_extracted_licensing_infos {
        *declared
            .entry(license.license_id.as_deref().unwrap_or("missing"))
            .or_default() += 1;
    }
    let duplicate: Vec<&str> = declared
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|(license_id, _)| *license_id)
        .collect();
    if !duplicate.is_empty() {
        result.error(
            "spdx-license-refs",
            format!(
                "{count} LicenseRefs are declared more than once, e.g. {examples}",
                count = duplicate.len(),
                examples = examples(&duplicate),
            ),
        );
    }

    let undeclared: BTreeSet<&str> = document
        .packages
        .iter()
        .flat_map(|package| [&package.license_declared, &package.license_concluded])
        .filter_map(|license| license.as_deref())
        .flat_map(|license| LICENSE_REF.find_iter(license).map(|found| found.as_str()))
        .filter(|license_ref| !declared.contains_key(license_ref))
        .collect();
    if !undeclared.is_empty() {
        result.error(
            "spdx-license-refs",
            format!(
                "{count} LicenseRefs are used but not declared in hasExtractedLicensingInfos, \
                 e.g. {examples}",
                count = undeclared.len(),
                examples = examples(&undeclared),
            ),
        );
    }
}

/// A license that is not a valid SPDX expression makes the document invalid for strict parsers,
/// and license scanners cannot use it.
fn check_license_expressions(result: &mut ImageResult, packages: &[Package]) {
    let licenses: BTreeSet<&str> = packages
        .iter()
        .flat_map(|package| [&package.license_declared, &package.license_concluded])
        .filter_map(|license| license.as_deref())
        .filter(|license| !["NOASSERTION", "NONE"].contains(license))
        .collect();
    let invalid: Vec<&str> = licenses
        .into_iter()
        .filter(|license| !is_valid_license(license))
        .collect();
    if !invalid.is_empty() {
        result.error(
            "spdx-license-expressions",
            format!(
                "{count} licenses are not valid SPDX expressions, e.g. {examples}",
                count = invalid.len(),
                examples = examples(&invalid),
            ),
        );
    }
}

fn check_relationships(
    result: &mut ImageResult,
    relationships: &[Relationship],
    by_id: &BTreeMap<&str, &Package>,
    root_id: &str,
) {
    let mut graph = BTreeMap::<&str, BTreeSet<&str>>::new();
    let mut dangling = BTreeSet::new();
    for relationship in relationships {
        let source = relationship.spdx_element_id.as_deref().unwrap_or("missing");
        let related = relationship
            .related_spdx_element
            .as_deref()
            .unwrap_or("missing");
        for element in [source, related] {
            if element != DOCUMENT_ID && !by_id.contains_key(element) {
                dangling.insert(element);
            }
        }
        if matches!(
            relationship.relationship_type.as_deref(),
            Some("CONTAINS" | "DEPENDS_ON")
        ) {
            graph.entry(source).or_default().insert(related);
        }
    }
    if !dangling.is_empty() {
        result.error(
            "spdx-relationships",
            format!(
                "{count} relationship elements do not exist, e.g. {examples}",
                count = dangling.len(),
                examples = examples(&dangling),
            ),
        );
    }

    let mut reachable = BTreeSet::new();
    let mut pending = vec![root_id];
    while let Some(spdx_id) = pending.pop() {
        if reachable.insert(spdx_id) {
            pending.extend(graph.get(spdx_id).into_iter().flatten());
        }
    }
    let orphans: BTreeSet<&str> = by_id
        .keys()
        .copied()
        .filter(|spdx_id| !reachable.contains(spdx_id))
        .collect();
    if !orphans.is_empty() {
        result.error(
            "spdx-relationships",
            format!(
                "{count} of {total} packages are not reachable from the image, e.g. {examples}",
                count = orphans.len(),
                total = by_id.len(),
                examples = examples(&orphans),
            ),
        );
    }
}
