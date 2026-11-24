//! Every check is tested the same way: a small pair of SBOMs that passes all of them is broken in
//! one place, and exactly that check has to notice.

use std::collections::BTreeMap;

use rstest::{fixture, rstest};
use serde_json::{Value, json};

use super::*;

const DIGEST: &str = "sha256:abababababababababababababababababababababababababababababababab";
const IMAGE_PURL: &str = "pkg:oci/kafka@sha256%3Aabababababababababababababababababababababababababababababababab?arch=amd64&repository_url=oci.stackable.tech%2Fsdp%2Fkafka";
const OTHER_IMAGE_PURL: &str = "pkg:oci/kafka@sha256%3Acdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcd?arch=amd64&repository_url=oci.stackable.tech%2Fsdp%2Fkafka";
const GLIBC: &str = "pkg:rpm/redhat/glibc@2.34-1.el9?arch=x86_64&distro=rhel-9.9";
const BASH: &str = "pkg:rpm/redhat/bash@5.1.8-1.el9?arch=x86_64&distro=rhel-9.9";
const KAFKA: &str = "pkg:maven/org.apache.kafka/kafka-clients@4.3.1-stackable0.0.0-dev";
const EMBEDDED_SBOM: &str = "/stackable/kafka_2.13-4.3.1-stackable0.0.0-dev/kafka.cdx.json";

fn target(architecture: &str) -> Target {
    Target {
        image: "kafka".to_owned(),
        version: Some("4.3.1".to_owned()),
        tag: "4.3.1-stackable0.0.0-dev".to_owned(),
        architecture: architecture.to_owned(),
    }
}

#[fixture]
fn expectations() -> Expectations {
    Expectations {
        registry: "oci.stackable.tech/sdp".to_owned(),
        release: "0.0.0-dev".to_owned(),
        cyclonedx_spec_version: "1.7".to_owned(),
        max_image_age_days: 7,
        operating_system: "redhat".to_owned(),
        max_without_purl: 0.05,
        max_without_version: 0.05,
        main_components: vec!["pkg:maven/org.apache.kafka/kafka-clients@{version}".to_owned()],
        embedded_sboms: vec![
            "/stackable/kafka_2.13-{version}-stackable{release}/*.cdx.json".to_owned(),
        ],
        min_components: BTreeMap::from([
            ("total".to_owned(), 4),
            ("rpm".to_owned(), 2),
            ("embedded-maven".to_owned(), 1),
        ]),
        min_license_coverage: Some(0.6),
        ignore: BTreeSet::new(),
    }
}

fn component(purl: &str, name: &str, version: &str, cataloger: &str, path: &str) -> Value {
    json!({
        "type": "library",
        "bom-ref": format!("{purl}&package-id=1"),
        "name": name,
        "version": version,
        "purl": purl,
        "licenses": [{ "license": { "id": "Apache-2.0" } }],
        "properties": [
            { "name": "syft:package:foundBy", "value": cataloger },
            { "name": "syft:location:0:path", "value": path },
        ],
    })
}

/// A CycloneDX SBOM as mergebom writes it, cut down to what the checks look at.
#[fixture]
fn cyclonedx() -> Value {
    let organization = json!({ "name": "Stackable GmbH", "url": ["https://stackable.tech/"] });
    let components = vec![
        json!({ "type": "operating-system", "bom-ref": "os", "name": "redhat", "version": "9.9" }),
        component(
            GLIBC,
            "glibc",
            "2.34-1.el9",
            "rpm-db-cataloger",
            "/var/lib/rpm/db",
        ),
        component(
            BASH,
            "bash",
            "5.1.8-1.el9",
            "rpm-db-cataloger",
            "/var/lib/rpm/db",
        ),
        component(
            KAFKA,
            "kafka-clients",
            "4.3.1-stackable0.0.0-dev",
            "sbom-cataloger",
            EMBEDDED_SBOM,
        ),
    ];
    let bom_ref = |index: usize| components[index]["bom-ref"].clone();
    json!({
        "bomFormat": "CycloneDX",
        "specVersion": "1.7",
        "serialNumber": "urn:uuid:972ce3e9-363b-4fb5-80c4-ba546dfc50e0",
        "version": 1,
        "metadata": {
            "timestamp": "2026-11-16T01:12:24Z",
            "authors": [{ "name": "Stackable GmbH", "email": "info@stackable.tech" }],
            "lifecycles": [{ "phase": "post-build" }],
            "tools": {
                "components": [
                    { "type": "application", "name": "syft", "version": "1.44.0" },
                    { "type": "application", "name": "mergebom", "version": "0.1.0" },
                ]
            },
            "component": {
                "type": "container",
                "bom-ref": "root",
                "name": "kafka",
                "version": "4.3.1-stackable0.0.0-dev-amd64",
                "purl": IMAGE_PURL,
                "supplier": organization,
                "manufacturer": organization,
                "description": "Apache Kafka.",
                "licenses": [{ "license": { "id": "Apache-2.0" } }],
            },
        },
        "dependencies": [
            { "ref": "root", "dependsOn": [bom_ref(0), bom_ref(1), bom_ref(2)] },
            { "ref": bom_ref(1), "dependsOn": [bom_ref(3)] },
        ],
        "components": components,
    })
}

/// The SPDX document that belongs to the CycloneDX SBOM.
#[fixture]
fn spdx() -> Value {
    let package = |spdx_id: &str, name: &str, purl: &str, supplier: &str| {
        json!({
            "SPDXID": spdx_id,
            "name": name,
            "versionInfo": "1",
            "supplier": supplier,
            "downloadLocation": "NOASSERTION",
            "licenseDeclared": "Apache-2.0 AND LicenseRef-Fedora-Public-Domain",
            "externalRefs": [{ "referenceType": "purl", "referenceLocator": purl }],
        })
    };
    let relationship = |source: &str, kind: &str, related: &str| json!({ "spdxElementId": source, "relationshipType": kind, "relatedSpdxElement": related });

    let mut root = package(
        "SPDXRef-Image",
        "kafka",
        IMAGE_PURL,
        "Organization: Stackable GmbH",
    );
    root["versionInfo"] = json!("4.3.1-stackable0.0.0-dev-amd64");
    root["primaryPackagePurpose"] = json!("CONTAINER");
    json!({
        "spdxVersion": "SPDX-2.3",
        "dataLicense": "CC0-1.0",
        "SPDXID": "SPDXRef-DOCUMENT",
        "documentNamespace": "https://stackable.tech/spdxdocs/kafka-1",
        "documentDescribes": null,
        "creationInfo": {
            "created": "2026-11-16T01:12:24Z",
            "creators": [
                "Organization: Stackable GmbH (info@stackable.tech)",
                "Tool: mergebom-0.1.0",
                "Tool: syft-1.44.0",
            ],
        },
        "packages": [
            root,
            package("SPDXRef-glibc", "glibc", GLIBC, "Organization: Red Hat, Inc."),
            package("SPDXRef-bash", "bash", BASH, "Organization: Red Hat, Inc."),
            package("SPDXRef-kafka-clients", "kafka-clients", KAFKA, "NOASSERTION"),
        ],
        "relationships": [
            relationship("SPDXRef-DOCUMENT", "DESCRIBES", "SPDXRef-Image"),
            relationship("SPDXRef-Image", "CONTAINS", "SPDXRef-glibc"),
            relationship("SPDXRef-Image", "CONTAINS", "SPDXRef-bash"),
            relationship("SPDXRef-glibc", "DEPENDS_ON", "SPDXRef-kafka-clients"),
        ],
        "hasExtractedLicensingInfos": [
            { "licenseId": "LicenseRef-Fedora-Public-Domain", "extractedText": "x" }
        ],
    })
}

fn run_checks(
    cyclonedx: Option<&Value>,
    spdx: Option<&Value>,
    expectations: &Expectations,
) -> ImageResult {
    let mut result = ImageResult::new(target("amd64"), Some(DIGEST.to_owned()));
    check_sboms(
        &mut result,
        cyclonedx.map(Value::to_string).as_deref(),
        spdx.map(Value::to_string).as_deref(),
        expectations,
    );
    result
}

fn checks(result: &ImageResult) -> BTreeSet<&'static str> {
    result
        .findings
        .iter()
        .map(|finding| finding.check)
        .collect()
}

fn array(value: &mut Value) -> &mut Vec<Value> {
    value.as_array_mut().expect("the value should be an array")
}

#[rstest]
fn good_sboms_have_no_findings(cyclonedx: Value, spdx: Value, expectations: Expectations) {
    let result = run_checks(Some(&cyclonedx), Some(&spdx), &expectations);
    let messages: Vec<&str> = result
        .findings
        .iter()
        .map(|finding| finding.message.as_str())
        .collect();
    assert_eq!(messages, Vec::<&str>::new());
}

#[rstest]
#[case::old_spec_version(|bom: &mut Value| bom["specVersion"] = json!("1.5"), "cdx-format")]
#[case::not_an_sbom(|bom: &mut Value| *bom = json!({ "components": "none" }), "cdx-format")]
#[case::sbom_of_another_digest(
    |bom: &mut Value| bom["metadata"]["component"]["purl"] = json!(OTHER_IMAGE_PURL),
    "cdx-root-identity"
)]
#[case::sbom_of_another_architecture(
    |bom: &mut Value| bom["metadata"]["component"]["version"] = json!("4.3.1-stackable0.0.0-dev-arm64"),
    "cdx-root-identity"
)]
#[case::no_supplier(|bom: &mut Value| bom["metadata"]["component"]["supplier"] = Value::Null, "cdx-root-metadata")]
#[case::no_license_on_the_image(
    |bom: &mut Value| bom["metadata"]["component"]["licenses"] = json!([]),
    "cdx-root-metadata"
)]
#[case::mergebom_did_not_run(
    |bom: &mut Value| { array(&mut bom["metadata"]["tools"]["components"]).pop(); },
    "cdx-document-metadata"
)]
#[case::no_components(|bom: &mut Value| bom["components"] = json!([]), "cdx-components")]
#[case::duplicate_bom_ref(
    |bom: &mut Value| bom["components"][2]["bom-ref"] = bom["components"][1]["bom-ref"].clone(),
    "cdx-schema"
)]
#[case::component_without_bom_ref(
    |bom: &mut Value| { bom["components"][2].as_object_mut().expect("the component should be an object").remove("bom-ref"); },
    "cdx-component-identity"
)]
#[case::file_components(
    |bom: &mut Value| array(&mut bom["components"]).push(json!({ "type": "file", "bom-ref": "f" })),
    "cdx-file-components"
)]
#[case::component_without_purl(|bom: &mut Value| bom["components"][1]["purl"] = Value::Null, "cdx-purl")]
#[case::malformed_purl(|bom: &mut Value| bom["components"][3]["purl"] = json!("maven:kafka-clients"), "cdx-schema")]
#[case::purl_with_duplicate_qualifier(
    |bom: &mut Value| bom["components"][1]["purl"] = json!(format!("{GLIBC}&arch=aarch64")),
    "cdx-schema"
)]
#[case::invalid_serial_number(|bom: &mut Value| bom["serialNumber"] = json!("nope"), "cdx-schema")]
#[case::invalid_license_id(
    |bom: &mut Value| bom["components"][1]["licenses"] = json!([{ "license": { "id": "Apache 2.0" } }]),
    "cdx-schema"
)]
#[case::invalid_license_expression(
    |bom: &mut Value| bom["components"][1]["licenses"] = json!([{ "expression": "Apache-2.0 ORR MIT" }]),
    "cdx-schema"
)]
#[case::wrong_case_in_license_exception(
    |bom: &mut Value| bom["components"][1]["licenses"] = json!([{ "expression": "Apache-2.0 WITH LLVM-Exception" }]),
    "cdx-schema"
)]
#[case::invalid_license_on_the_image(
    |bom: &mut Value| bom["metadata"]["component"]["licenses"] = json!([{ "expression": "Apache 2.0" }]),
    "cdx-schema"
)]
#[case::malformed_hash(
    |bom: &mut Value| bom["components"][1]["hashes"] = json!([{ "alg": "SHA-256", "content": "xyz" }]),
    "cdx-schema"
)]
#[case::unknown_component_type(|bom: &mut Value| bom["components"][1]["type"] = json!("banana"), "cdx-schema")]
#[case::component_without_version(
    |bom: &mut Value| bom["components"][1]["version"] = json!("UNKNOWN"),
    "cdx-component-version"
)]
#[case::duplicate_purl(
    |bom: &mut Value| {
        let mut duplicate = bom["components"][3].clone();
        duplicate["bom-ref"] = json!("other");
        array(&mut bom["components"]).push(duplicate);
    },
    "cdx-duplicate-purls"
)]
#[case::superseded_rpm_from_a_lower_layer(
    |bom: &mut Value| {
        let mut old = component(
            &GLIBC.replace("2.34-1", "2.34-0"),
            "glibc",
            "2.34-0.el9",
            "rpm-db-cataloger",
            "/var/lib/rpm/db",
        );
        old["bom-ref"] = json!("old-glibc");
        array(&mut bom["components"]).push(old);
    },
    "cdx-rpm-versions"
)]
#[case::syft_bookkeeping_left_in(
    |bom: &mut Value| array(&mut bom["components"][1]["properties"])
        .push(json!({ "name": "syft:location:0:layerID", "value": "sha256:1" })),
    "cdx-syft-properties"
)]
#[case::no_operating_system(|bom: &mut Value| { array(&mut bom["components"]).remove(0); }, "cdx-operating-system")]
#[case::rpm_without_distro(
    |bom: &mut Value| bom["components"][1]["purl"] = json!("pkg:rpm/redhat/glibc@2.34-1.el9?arch=x86_64"),
    "cdx-operating-system"
)]
#[case::no_dependencies(|bom: &mut Value| bom["dependencies"] = json!([]), "cdx-dependency-graph")]
#[case::dangling_dependency(
    |bom: &mut Value| array(&mut bom["dependencies"][0]["dependsOn"]).push(json!("gone")),
    "cdx-schema"
)]
#[case::component_outside_the_graph(
    |bom: &mut Value| { array(&mut bom["dependencies"]).pop(); },
    "cdx-dependency-graph"
)]
#[case::wrong_product_version(
    |bom: &mut Value| bom["components"][3]["purl"] = json!(KAFKA.replace("4.3.1", "4.3.10")),
    "main-component"
)]
#[case::build_time_sbom_lost(|bom: &mut Value| bom["components"][3]["properties"] = json!([]), "embedded-sbom")]
#[case::few_licenses(
    |bom: &mut Value| array(&mut bom["components"]).iter_mut().for_each(|component| component["licenses"] = json!([])),
    "license-coverage"
)]
fn cyclonedx_defects(
    mut cyclonedx: Value,
    expectations: Expectations,
    #[case] damage: fn(&mut Value),
    #[case] check: &str,
) {
    damage(&mut cyclonedx);
    let result = run_checks(Some(&cyclonedx), None, &expectations);
    assert!(
        checks(&result).contains(check),
        "{check} should report {findings:?}",
        findings = result.findings
    );
}

#[rstest]
#[case::syft_namespace(|document: &mut Value| document["documentNamespace"] = json!("https://anchore.com/syft/x"), "spdx-format")]
#[case::creator_without_email(
    |document: &mut Value| document["creationInfo"]["creators"][0] = json!("Organization: Stackable GmbH"),
    "spdx-creation-info"
)]
#[case::duplicate_spdx_id(|document: &mut Value| document["packages"][2]["SPDXID"] = json!("SPDXRef-glibc"), "spdx-packages")]
#[case::nothing_described(|document: &mut Value| { array(&mut document["relationships"]).remove(0); }, "spdx-root")]
#[case::image_of_another_digest(
    |document: &mut Value| document["packages"][0]["externalRefs"][0]["referenceLocator"] = json!(OTHER_IMAGE_PURL),
    "spdx-root"
)]
#[case::stackable_as_supplier_of_everything(
    |document: &mut Value| array(&mut document["packages"])
        .iter_mut()
        .for_each(|package| package["supplier"] = json!("Organization: Stackable GmbH")),
    "spdx-supplier"
)]
#[case::invalid_license_expression(
    |document: &mut Value| document["packages"][1]["licenseDeclared"] = json!("Apache 2.0"),
    "spdx-license-expressions"
)]
#[case::wrong_case_in_license_exception(
    |document: &mut Value| document["packages"][1]["licenseDeclared"] = json!("Apache-2.0 WITH LLVM-Exception"),
    "spdx-license-expressions"
)]
#[case::undeclared_license_ref(|document: &mut Value| document["hasExtractedLicensingInfos"] = json!([]), "spdx-license-refs")]
#[case::package_outside_the_graph(|document: &mut Value| { array(&mut document["relationships"]).pop(); }, "spdx-relationships")]
#[case::package_missing(
    |document: &mut Value| {
        array(&mut document["packages"]).pop();
        array(&mut document["relationships"]).pop();
    },
    "spdx-matches-cyclonedx"
)]
fn spdx_defects(
    cyclonedx: Value,
    mut spdx: Value,
    expectations: Expectations,
    #[case] damage: fn(&mut Value),
    #[case] check: &str,
) {
    damage(&mut spdx);
    let result = run_checks(Some(&cyclonedx), Some(&spdx), &expectations);
    assert!(
        checks(&result).contains(check),
        "{check} should report {findings:?}",
        findings = result.findings
    );
}

/// An invalid license on every RPM is one problem and should read as one finding.
#[rstest]
fn schema_errors_are_grouped_by_field(mut cyclonedx: Value, expectations: Expectations) {
    for index in [1, 2] {
        cyclonedx["components"][index]["licenses"] = json!([{ "license": { "id": "Apache 2.0" } }]);
    }
    let result = run_checks(Some(&cyclonedx), None, &expectations);

    let fields: Vec<&str> = result
        .findings
        .iter()
        .filter(|finding| finding.check == "cdx-schema")
        .filter_map(|finding| finding.message.split_once(": 2 errors, e.g. "))
        .map(|(field, _)| field)
        .collect();
    assert_eq!(
        fields,
        ["components[N].licenses[N].license.license_identifier.SpdxId"]
    );
}

/// Deprecated IDs are still on the SPDX license list, mergebom keeps them on purpose.
#[rstest]
fn deprecated_license_ids_are_fine(
    mut cyclonedx: Value,
    mut spdx: Value,
    expectations: Expectations,
) {
    cyclonedx["components"][1]["licenses"] =
        json!([{ "expression": "MIT AND BSD-2-Clause-FreeBSD" }]);
    cyclonedx["components"][2]["licenses"] = json!([{ "license": { "id": "GPL-2.0" } }]);
    spdx["packages"][1]["licenseDeclared"] = json!("MIT AND BSD-2-Clause-FreeBSD");

    let result = run_checks(Some(&cyclonedx), Some(&spdx), &expectations);
    assert_eq!(checks(&result), BTreeSet::new());
}

#[rstest]
fn component_floors(cyclonedx: Value, mut expectations: Expectations) {
    expectations.min_components = BTreeMap::from([
        ("total".to_owned(), 5),
        ("embedded-maven".to_owned(), 2),
        ("pypi".to_owned(), 1),
    ]);
    let result = run_checks(Some(&cyclonedx), None, &expectations);

    let messages: Vec<&str> = result
        .findings
        .iter()
        .filter(|finding| finding.check == "component-count")
        .map(|finding| finding.message.as_str())
        .collect();
    assert_eq!(
        messages,
        [
            "1 maven components from build-time SBOMs, expected at least 2",
            "0 pypi components, expected at least 1",
            "4 components in total, expected at least 5",
        ]
    );
}

#[rstest]
fn vendored_rpms_of_another_distribution_are_fine(
    mut cyclonedx: Value,
    expectations: Expectations,
) {
    // A Python wheel that bundles AlmaLinux libraries and says so in its own SBOM.
    let vendored = component(
        "pkg:rpm/almalinux/glibc@2.28-1.el8",
        "glibc",
        "2.28-1.el8",
        "sbom-cataloger",
        "/stackable/app/x.dist-info/sboms/auditwheel.cdx.json",
    );
    array(&mut cyclonedx["dependencies"][0]["dependsOn"]).push(vendored["bom-ref"].clone());
    array(&mut cyclonedx["components"]).push(vendored);

    let result = run_checks(Some(&cyclonedx), None, &expectations);
    assert_eq!(checks(&result), BTreeSet::new());
}

#[rstest]
fn other_operating_system(cyclonedx: Value, mut expectations: Expectations) {
    expectations.operating_system = "rocky".to_owned();
    let result = run_checks(Some(&cyclonedx), None, &expectations);
    assert_eq!(checks(&result), BTreeSet::from(["cdx-operating-system"]));
}

#[rstest]
fn old_image_is_a_warning(mut cyclonedx: Value, expectations: Expectations) {
    cyclonedx["metadata"]["properties"] = json!([{
        "name": "syft:image:labels:org.opencontainers.image.created",
        "value": "2020-01-01T00:00:00.123456789Z",
    }]);
    let result = run_checks(Some(&cyclonedx), None, &expectations);

    let findings: Vec<(Severity, &str)> = result
        .findings
        .iter()
        .map(|finding| (finding.severity, finding.check))
        .collect();
    assert_eq!(findings, [(Severity::Warning, "image-age")]);
}

#[rstest]
fn ignore_findings(mut cyclonedx: Value, mut expectations: Expectations) {
    cyclonedx["specVersion"] = json!("1.5");
    let mut result = run_checks(Some(&cyclonedx), None, &expectations);

    expectations.ignore.insert("cdx-format".to_owned());
    apply_ignores(&mut result, &expectations);

    assert_eq!((result.findings.len(), result.ignored), (0, 1));
}

#[rstest]
#[case::same_components_in_other_versions(
    &[GLIBC, KAFKA],
    &["pkg:rpm/redhat/glibc@2.34-2.el9?arch=aarch64&distro=rhel-9.9", KAFKA],
    false
)]
#[case::half_the_components_missing(
    &["pkg:maven/a/a@1", "pkg:maven/b/b@1", "pkg:maven/c/c@1", "pkg:maven/d/d@1"],
    &["pkg:maven/a/a@1", "pkg:maven/b/b@1"],
    true
)]
fn compare_architectures(#[case] amd64: &[&str], #[case] arm64: &[&str], #[case] differ: bool) {
    let identity = |purls: &[&str]| -> Identity {
        purls
            .iter()
            .filter_map(|purl| Purl::parse(purl))
            .map(|purl| (purl.package_type, purl.name))
            .collect()
    };
    let mut results = vec![
        ImageResult::new(target("amd64"), None),
        ImageResult::new(target("arm64"), None),
    ];
    let identities = BTreeMap::from([
        (target("amd64"), identity(amd64)),
        (target("arm64"), identity(arm64)),
    ]);

    check_architectures_agree(&mut results, &identities);

    let reported: Vec<BTreeSet<&str>> = results.iter().map(checks).collect();
    let expected = if differ {
        BTreeSet::from(["architectures-agree"])
    } else {
        BTreeSet::new()
    };
    assert_eq!(reported, [BTreeSet::new(), expected]);
}

#[rstest]
#[case("4.3.1", Some("4.3.1"), true)]
#[case("4.3.1-stackable0.0.0-dev", Some("4.3.1"), true)]
#[case("v1.20.2-stackable0.0.0-dev%2Bdirty", Some("v1.20.2"), true)]
#[case("4.3.10", Some("4.3.1"), false)]
#[case("4.3.1.1", Some("4.3.1"), false)]
#[case("4.3.0", Some("4.3.1"), false)]
#[case("21.0.13.0.0.9-1", None, true)]
fn match_versions(#[case] actual: &str, #[case] expected: Option<&str>, #[case] matches: bool) {
    assert_eq!(matches_version(actual, expected), matches);
}

#[rstest]
fn build_expected_image_purl() {
    assert_eq!(
        expected_image_purl("oci.stackable.tech/sdp", &target("amd64"), DIGEST),
        IMAGE_PURL
    );
}

#[rstest]
#[case::few(&["b", "a"], "a, b")]
#[case::many(&["a", "b", "c", "d", "e"], "a, b, c and 2 more")]
#[case::duplicates(&["a", "a"], "a")]
fn sample_examples(#[case] items: &[&str], #[case] sample: &str) {
    assert_eq!(examples(items), sample);
}
