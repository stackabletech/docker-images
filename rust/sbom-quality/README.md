# SBOM quality check

`sbom-quality` checks the SBOMs that are attested to our published images: every product version
and every operator, both architectures, CycloneDX and SPDX. It is meant to run daily against the
`0.0.0-dev` images, so that a change that makes the SBOMs worse is noticed within a day instead
of at the next manual audit.

The SBOMs are produced when an image is published, by syft and mergebom in the `publish-image`
action of [stackabletech/actions](https://github.com/stackabletech/actions), from the image
contents and from the SBOMs that the product builds embed into the image. A Dockerfile change, a
new product version, a syft update or a failing attestation step can all break them without
breaking the build.

## Usage

Run it from the root of the repository:

```shell
cargo sbom-quality                         # everything in expectations.toml
cargo sbom-quality kafka opa=1.20.2        # some products, versions or operators
cargo sbom-quality --arch amd64 --verbose  # one architecture, every finding and image
cargo sbom-quality --format json           # for a machine
cargo sbom-quality --cache-dir /tmp/sboms  # keep the attestations between runs
```

It needs `crane` and `cosign` on the `PATH` and no registry credentials. A full run fetches
about 300 attestations and takes a few minutes. Attestations are immutable per image
digest, so with `--cache-dir` a second run takes seconds.

The exit code is 1 if there is at least one error, warnings do not change it. It is 2 if the
check itself fails, for example on an invalid expectations file.

Two local files can be checked instead of the registry, for example the output of mergebom
before it is published:

```shell
cargo sbom-quality kafka=4.3.1 --arch amd64 \
  --cyclonedx sbom.cdx.json --spdx sbom.spdx.json --digest sha256:...
```

## Daily run

The [SBOM quality](../../.github/workflows/sbom_quality.yaml) workflow runs the check every day
at 07:30 UTC, after the scheduled image builds, and can be started by hand. A run fails if there
is at least one error finding, so the status of the latest run is the status of our SBOMs. It
does not notify anyone, something outside has to watch it.

A failed run shows the summary per check on its summary page and in the log. The log also has
every finding with every affected image, in the collapsed group "Every finding".

## What is checked

Invariants, the same for every image. Most of them are defects we had at some point.

| Check | What it guards |
| --- | --- |
| `image-missing` | The tag exists and has an image for every architecture. |
| `attestation-cyclonedx`, `attestation-spdxjson` | The attestation exists and cosign verifies it against our GitHub workflows. |
| `cdx-format`, `spdx-format` | Format versions, our own SPDX namespace. |
| `cdx-schema` | The document is valid CycloneDX according to the [cyclonedx-bom](https://crates.io/crates/cyclonedx-bom) crate: serial number, SPDX license IDs and expressions, purls, hashes, enum values, unique bom-refs, no dangling dependencies. One finding per field, not per component. |
| `cdx-root-identity`, `spdx-root` | The SBOM describes exactly this image: name, tag, architecture and digest. |
| `cdx-root-metadata`, `cdx-document-metadata`, `spdx-creation-info` | Supplier, manufacturer, license, description, authors, lifecycle, syft and mergebom as tools. |
| `cdx-component-identity`, `cdx-component-version`, `cdx-purl`, `spdx-packages` | Components have a name, a bom-ref, a version and a purl. |
| `cdx-duplicate-purls` | mergebom merged the components that syft reports twice. |
| `cdx-rpm-versions` | No RPM in two versions, which is what scanning all layers instead of the final filesystem produces. |
| `cdx-file-components`, `cdx-syft-properties`, `cdx-size` (warning) | Nothing that only bloats the document. Rekor rejects attestations that get too large. |
| `cdx-operating-system` | One operating system component, and installed RPMs carry its namespace and the `distro` qualifier. |
| `cdx-dependency-graph`, `spdx-relationships` | Every component is reachable from the image, and SPDX relationships point to existing packages. |
| `spdx-supplier` | We are the supplier of the image, not of the packages in it. |
| `spdx-license-refs` | Every `LicenseRef-` is declared. Strict SPDX parsers reject the document otherwise. |
| `spdx-license-expressions` | Every license is a valid SPDX expression according to the [spdx](https://crates.io/crates/spdx) crate. |
| `spdx-matches-cyclonedx` | Both documents list the same packages. |
| `architectures-agree` | amd64 and arm64 list the same components (names, not versions). |
| `image-age` (warning) | The image is not older than the scheduled builds allow. |

Expectations per product, from `expectations.toml`. These notice what the invariants cannot: an
SBOM without the Python packages of Airflow is still a valid SBOM.

| Check | What it guards |
| --- | --- |
| `main-component` | The product itself is listed, in the version the image is tagged with. |
| `embedded-sbom` | Components come from each build-time SBOM that the image is supposed to contain. |
| `component-count` | At least so many components in total, per ecosystem (`maven`) and per ecosystem from build-time SBOMs (`embedded-maven`). |
| `license-coverage` | At least this share of the components has a license. |
| `expectations` | Every product that boil builds is listed, and no expectation is left for a removed version. |

Licenses are checked as strictly as mergebom writes them, in both documents: deprecated SPDX IDs
like `BSD-2-Clause-FreeBSD` are valid, since they are still on the license list. IDs have to be
spelled as on the license list. SPDX itself matches them case-insensitively, but the spdx crate
and cyclonedx-bom do not, so mergebom fixes the case.

The counts from build-time SBOMs are separate because syft finds most jars and Python packages
on its own as well. If the build-time SBOM of Kafka is lost, the number of Maven components
hardly changes, but the purls get worse and everything shaded disappears.

## Expectations

`expectations.toml` documents its own format at the top. The component floors are generated:

```shell
cargo sbom-quality --suggest-floors          # all products
cargo sbom-quality --suggest-floors airflow  # one product
```

This prints tables to paste, with the smallest count over all versions and architectures minus
20%. It also suggests `embedded-sboms`, but review that list: mergebom keeps one location per
merged component, so a small build-time SBOM that overlaps with a bigger one may have no
component left that points to it.

When a check fails:

- Something broke: fix the image or the pipeline.
- A count dropped for a good reason (a dependency cleanup, a new major version): lower the floor.
  If only one version differs, use `[products.<name>.versions."<version>"]`.
- A deviation is known and accepted: add the check ID to `ignore` of the product, with a comment.

A new product needs a `[products.<name>]` table (or an entry in `unchecked`), the check fails
until it has one. A new version of an existing product needs nothing.

## Tests

```shell
cargo test -p sbom-quality
```
