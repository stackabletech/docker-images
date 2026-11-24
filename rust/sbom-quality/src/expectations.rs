//! The expectations file and the list of images to check.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu, ensure};

/// Where the expectations file is, relative to the docker-images repository.
pub const DEFAULT_PATH: &str = "rust/sbom-quality/expectations.toml";

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read the expectations file {path:?}"))]
    ReadExpectations {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to parse the expectations file {path:?}"))]
    ParseExpectations {
        source: toml::de::Error,
        path: PathBuf,
    },

    #[snafu(display("{table:?} has `images`, which only [operators] may have"))]
    ImagesOutsideOperators { table: String },

    #[snafu(display("{table:?} has `versions`, which only a product may have"))]
    UnexpectedVersions { table: String },

    #[snafu(display("{table:?} has the invalid embedded SBOM pattern {pattern:?}"))]
    InvalidPattern {
        source: glob::PatternError,
        table: String,
        pattern: String,
    },

    #[snafu(display("failed to read the boil configuration {path:?}"))]
    ReadBoilConfig {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to parse the boil configuration {path:?}"))]
    ParseBoilConfig {
        source: toml::de::Error,
        path: PathBuf,
    },

    #[snafu(display("{name:?} is neither a product nor an operator in the expectations"))]
    UnknownImage { name: String },

    #[snafu(display("{product:?} has no version {version:?} in its boil-config.toml"))]
    UnknownVersion { product: String, version: String },
}

/// The expectations file. Its top-level settings apply to every image.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct ExpectationsFile {
    pub registry: String,
    pub release: String,
    pub architectures: Vec<String>,

    #[serde(default = "default_certificate_identity_regexp")]
    pub certificate_identity_regexp: String,

    #[serde(default = "default_certificate_oidc_issuer")]
    pub certificate_oidc_issuer: String,

    pub cyclonedx_spec_version: String,
    pub max_image_age_days: i64,
    pub operating_system: String,
    pub max_without_purl: f64,
    pub max_without_version: f64,

    #[serde(default)]
    pub ignore: Vec<String>,

    /// Directories with a boil-config.toml whose images are not published.
    #[serde(default)]
    pub unchecked: Vec<String>,

    #[serde(default)]
    pub operators: Layer,

    #[serde(default)]
    pub products: BTreeMap<String, Layer>,
}

fn default_certificate_identity_regexp() -> String {
    r"^https://github.com/stackabletech/.+/.github/workflows/.+@.+".to_owned()
}

fn default_certificate_oidc_issuer() -> String {
    "https://token.actions.githubusercontent.com".to_owned()
}

/// Expectations of `[operators]`, a `[products.<name>]` or one of their versions. Each one
/// overrides the layer below it.
#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Layer {
    /// The operator images, only in `[operators]`.
    pub images: Vec<String>,

    pub main_component: Option<OneOrMany>,
    pub embedded_sboms: Option<Vec<String>>,

    /// Merged with the floors of the layer below, key by key.
    pub min_components: BTreeMap<String, u64>,

    pub min_license_coverage: Option<f64>,
    pub operating_system: Option<String>,
    pub max_without_purl: Option<f64>,
    pub max_without_version: Option<f64>,

    /// Added to the ignored checks of the layer below.
    pub ignore: Vec<String>,

    /// Layers for single product versions, only in `[products.<name>]`.
    pub versions: BTreeMap<String, Layer>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany {
    One(String),
    Many(Vec<String>),
}

impl OneOrMany {
    fn to_vec(&self) -> Vec<String> {
        match self {
            Self::One(item) => vec![item.clone()],
            Self::Many(items) => items.clone(),
        }
    }
}

/// Everything that is expected from the SBOMs of one image.
#[derive(Debug, Clone)]
pub struct Expectations {
    pub registry: String,
    pub release: String,
    pub cyclonedx_spec_version: String,
    pub max_image_age_days: i64,
    pub operating_system: String,
    pub max_without_purl: f64,
    pub max_without_version: f64,

    /// Purls of the product, see [`crate::check::substitute`] for the placeholders.
    pub main_components: Vec<String>,

    /// Paths of build-time SBOMs, with placeholders and `*` wildcards.
    pub embedded_sboms: Vec<String>,

    pub min_components: BTreeMap<String, u64>,
    pub min_license_coverage: Option<f64>,
    pub ignore: BTreeSet<String>,
}

impl Expectations {
    fn apply(&mut self, layer: &Layer) {
        if let Some(main_component) = &layer.main_component {
            self.main_components = main_component.to_vec();
        }
        if let Some(embedded_sboms) = &layer.embedded_sboms {
            self.embedded_sboms.clone_from(embedded_sboms);
        }
        if let Some(operating_system) = &layer.operating_system {
            self.operating_system.clone_from(operating_system);
        }
        self.min_components.extend(
            layer
                .min_components
                .iter()
                .map(|(kind, minimum)| (kind.clone(), *minimum)),
        );
        self.min_license_coverage = layer.min_license_coverage.or(self.min_license_coverage);
        self.max_without_purl = layer.max_without_purl.unwrap_or(self.max_without_purl);
        self.max_without_version = layer
            .max_without_version
            .unwrap_or(self.max_without_version);
        self.ignore.extend(layer.ignore.iter().cloned());
    }
}

/// One architecture of one image tag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Target {
    pub image: String,

    /// The product version, `None` for images that are only tagged with the release (operators).
    pub version: Option<String>,

    pub tag: String,
    pub architecture: String,
}

impl Target {
    pub fn canonical_tag(&self) -> String {
        format!(
            "{tag}-{architecture}",
            tag = self.tag,
            architecture = self.architecture
        )
    }
}

impl fmt::Display for Target {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{image}:{tag}",
            image = self.image,
            tag = self.canonical_tag()
        )
    }
}

impl ExpectationsFile {
    pub fn load(path: &Path) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path).context(ReadExpectationsSnafu { path })?;
        let file: Self = toml::from_str(&content).context(ParseExpectationsSnafu { path })?;
        file.validate()?;
        Ok(file)
    }

    /// Rejects what the format allows but what makes no sense, so it is not silently ignored.
    fn validate(&self) -> Result<(), Error> {
        let mut layers = vec![("operators".to_owned(), &self.operators)];
        ensure!(
            self.operators.versions.is_empty(),
            UnexpectedVersionsSnafu { table: "operators" }
        );

        for (product, layer) in &self.products {
            let table = format!("products.{product}");
            ensure!(
                layer.images.is_empty(),
                ImagesOutsideOperatorsSnafu { table: &table }
            );
            for (version, version_layer) in &layer.versions {
                let version_table = format!("{table}.versions.{version}");
                ensure!(
                    version_layer.images.is_empty(),
                    ImagesOutsideOperatorsSnafu {
                        table: &version_table
                    }
                );
                ensure!(
                    version_layer.versions.is_empty(),
                    UnexpectedVersionsSnafu {
                        table: &version_table
                    }
                );
                layers.push((version_table, version_layer));
            }
            layers.push((table, layer));
        }

        for (table, layer) in layers {
            for pattern in layer.embedded_sboms.iter().flatten() {
                glob::Pattern::new(pattern).context(InvalidPatternSnafu {
                    table: &table,
                    pattern,
                })?;
            }
        }
        Ok(())
    }

    /// The expectations for one image: the top-level settings, overridden by the ones of the
    /// product or of the operators, overridden by the ones of the product version.
    pub fn for_image(&self, image: &str, version: Option<&str>) -> Expectations {
        let layer = self.products.get(image).unwrap_or(&self.operators);
        let version_layer = version.and_then(|version| layer.versions.get(version));

        let mut expectations = Expectations {
            registry: self.registry.clone(),
            release: self.release.clone(),
            cyclonedx_spec_version: self.cyclonedx_spec_version.clone(),
            max_image_age_days: self.max_image_age_days,
            operating_system: self.operating_system.clone(),
            max_without_purl: self.max_without_purl,
            max_without_version: self.max_without_version,
            main_components: Vec::new(),
            embedded_sboms: Vec::new(),
            min_components: BTreeMap::new(),
            min_license_coverage: None,
            ignore: self.ignore.iter().cloned().collect(),
        };
        for layer in std::iter::once(layer).chain(version_layer) {
            expectations.apply(layer);
        }
        expectations
    }

    /// Every image architecture to check, optionally narrowed down by `name` or `name=version`
    /// selectors. Also returns the problems with the expectations found along the way.
    pub fn select_targets(
        &self,
        repository: &Path,
        selectors: &[String],
        architectures: &[String],
    ) -> Result<(Vec<Target>, Vec<String>), Error> {
        let mut wanted = BTreeMap::<&str, BTreeSet<Option<&str>>>::new();
        for selector in selectors {
            let (name, version) = match selector.split_once('=') {
                Some((name, version)) => (name, Some(version)),
                None => (selector.as_str(), None),
            };
            ensure!(
                self.products.contains_key(name)
                    || self.operators.images.iter().any(|image| image == name),
                UnknownImageSnafu { name }
            );
            wanted.entry(name).or_default().insert(version);
        }
        let is_wanted = |name: &str, version: Option<&str>| {
            selectors.is_empty()
                || wanted
                    .get(name)
                    .is_some_and(|versions| versions.contains(&None) || versions.contains(&version))
        };

        let mut problems = Vec::new();
        let mut tags = Vec::new();
        for (product, layer) in &self.products {
            if !selectors.is_empty() && !wanted.contains_key(product.as_str()) {
                continue;
            }

            let versions = product_versions(repository, product)?;
            for version in &versions {
                if is_wanted(product, Some(version)) {
                    let tag = format!("{version}-stackable{release}", release = self.release);
                    tags.push((product.clone(), Some(version.clone()), tag));
                }
            }

            for version in wanted.get(product.as_str()).into_iter().flatten().flatten() {
                ensure!(
                    versions.iter().any(|known| known == version),
                    UnknownVersionSnafu {
                        product,
                        version: *version
                    }
                );
            }

            // Expectations for a removed version are dead weight that nobody cleans up.
            for version in layer.versions.keys() {
                if !versions.contains(version) {
                    problems.push(format!(
                        "[products.{product}.versions] has expectations for {version}, which \
                         boil does not build"
                    ));
                }
            }
        }
        for operator in &self.operators.images {
            if is_wanted(operator, None) {
                tags.push((operator.clone(), None, self.release.clone()));
            }
        }

        if selectors.is_empty() {
            problems.extend(self.unknown_products(repository)?);
        }

        let targets = tags
            .into_iter()
            .flat_map(|(image, version, tag)| {
                architectures.iter().map(move |architecture| Target {
                    image: image.clone(),
                    version: version.clone(),
                    tag: tag.clone(),
                    architecture: architecture.clone(),
                })
            })
            .collect();
        Ok((targets, problems))
    }

    /// A new product must not slip through because nobody added it to the expectations.
    fn unknown_products(&self, repository: &Path) -> Result<Vec<String>, Error> {
        let entries =
            std::fs::read_dir(repository).context(ReadBoilConfigSnafu { path: repository })?;
        let mut built = BTreeSet::new();
        for entry in entries {
            let entry = entry.context(ReadBoilConfigSnafu { path: repository })?;
            if entry.path().join("boil-config.toml").is_file() {
                built.insert(entry.file_name().to_string_lossy().into_owned());
            }
        }

        Ok(built
            .into_iter()
            .filter(|name| !self.products.contains_key(name) && !self.unchecked.contains(name))
            .map(|name| {
                format!("{name} is built by boil but is neither in [products] nor in `unchecked`")
            })
            .collect())
    }
}

/// The versions of a product that boil builds.
fn product_versions(repository: &Path, product: &str) -> Result<Vec<String>, Error> {
    let path = repository.join(product).join("boil-config.toml");
    let content = std::fs::read_to_string(&path).context(ReadBoilConfigSnafu { path: &path })?;
    let config: toml::Table =
        toml::from_str(&content).context(ParseBoilConfigSnafu { path: &path })?;
    Ok(config
        .get("versions")
        .and_then(toml::Value::as_table)
        .map(|versions| versions.keys().cloned().collect())
        .unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    const EXPECTATIONS: &str = r#"
        registry = "oci.stackable.tech/sdp"
        release = "0.0.0-dev"
        architectures = ["amd64", "arm64"]
        cyclonedx-spec-version = "1.7"
        max-image-age-days = 7
        operating-system = "redhat"
        max-without-purl = 0.05
        max-without-version = 0.05
        unchecked = ["stackable-devel"]

        [operators]
        images = ["kafka-operator"]
        min-components = { total = 300 }

        [products.kafka]
        ignore = ["cdx-size"]
        min-license-coverage = 0.9
        min-components = { total = 1000, maven = 100 }

        [products.kafka.versions."3.9.2"]
        min-license-coverage = 0.5
        min-components = { maven = 50 }
    "#;

    #[fixture]
    fn expectations() -> ExpectationsFile {
        let file: ExpectationsFile =
            toml::from_str(EXPECTATIONS).expect("the expectations should parse");
        file.validate().expect("the expectations should be valid");
        file
    }

    /// A docker-images checkout with the given products and their versions.
    fn repository(products: &[(&str, &[&str])]) -> tempfile::TempDir {
        let directory = tempfile::tempdir().expect("a temporary directory should be created");
        for (product, versions) in products {
            let path = directory.path().join(product);
            std::fs::create_dir(&path).expect("the product directory should be created");
            let tables: String = versions
                .iter()
                .map(|version| format!("[versions.\"{version}\"]\n"))
                .collect();
            std::fs::write(path.join("boil-config.toml"), tables)
                .expect("the boil configuration should be written");
        }
        directory
    }

    #[rstest]
    fn layers_override_each_other(expectations: ExpectationsFile) {
        let product = expectations.for_image("kafka", Some("4.3.1"));
        let version = expectations.for_image("kafka", Some("3.9.2"));
        let operator = expectations.for_image("kafka-operator", None);

        assert_eq!(
            product.min_components,
            BTreeMap::from([("maven".to_owned(), 100), ("total".to_owned(), 1000)])
        );
        assert_eq!(
            version.min_components,
            BTreeMap::from([("maven".to_owned(), 50), ("total".to_owned(), 1000)])
        );
        assert_eq!(product.min_license_coverage, Some(0.9));
        assert_eq!(version.min_license_coverage, Some(0.5));
        assert_eq!(version.ignore, BTreeSet::from(["cdx-size".to_owned()]));
        assert_eq!(
            operator.min_components,
            BTreeMap::from([("total".to_owned(), 300)])
        );
        assert_eq!(operator.operating_system, "redhat");
    }

    #[rstest]
    #[case::unknown_key("[products.kafka]\nmin-component = 1")]
    #[case::images_in_product("[products.kafka]\nimages = [\"a\"]")]
    #[case::nested_versions("[products.kafka.versions.\"1\".versions.\"2\"]")]
    #[case::invalid_pattern("[products.kafka]\nembedded-sboms = [\"/a/[b\"]")]
    fn reject_invalid_expectations(#[case] addition: &str) {
        let content = format!("{EXPECTATIONS}\n{addition}");
        let parsed = toml::from_str::<ExpectationsFile>(&content)
            .map_err(|error| error.to_string())
            .and_then(|file| file.validate().map_err(|error| error.to_string()));
        assert!(parsed.is_err(), "{addition:?} should be rejected");
    }

    #[rstest]
    #[case::everything(
        &[],
        &[
            "kafka:3.9.2-stackable0.0.0-dev-amd64",
            "kafka:3.9.2-stackable0.0.0-dev-arm64",
            "kafka:4.3.1-stackable0.0.0-dev-amd64",
            "kafka:4.3.1-stackable0.0.0-dev-arm64",
            "kafka-operator:0.0.0-dev-amd64",
            "kafka-operator:0.0.0-dev-arm64",
        ]
    )]
    #[case::product(
        &["kafka"],
        &[
            "kafka:3.9.2-stackable0.0.0-dev-amd64",
            "kafka:3.9.2-stackable0.0.0-dev-arm64",
            "kafka:4.3.1-stackable0.0.0-dev-amd64",
            "kafka:4.3.1-stackable0.0.0-dev-arm64",
        ]
    )]
    #[case::version_and_operator(
        &["kafka=4.3.1", "kafka-operator"],
        &[
            "kafka:4.3.1-stackable0.0.0-dev-amd64",
            "kafka:4.3.1-stackable0.0.0-dev-arm64",
            "kafka-operator:0.0.0-dev-amd64",
            "kafka-operator:0.0.0-dev-arm64",
        ]
    )]
    fn select_targets(
        expectations: ExpectationsFile,
        #[case] selectors: &[&str],
        #[case] expected: &[&str],
    ) {
        let repository = repository(&[("kafka", &["3.9.2", "4.3.1"]), ("stackable-devel", &["1"])]);
        let selectors: Vec<String> = selectors.iter().map(ToString::to_string).collect();
        let architectures = ["amd64".to_owned(), "arm64".to_owned()];

        let (targets, problems) = expectations
            .select_targets(repository.path(), &selectors, &architectures)
            .expect("the targets should be selected");

        let targets: Vec<String> = targets.iter().map(ToString::to_string).collect();
        assert_eq!(targets, expected);
        assert_eq!(problems, Vec::<String>::new());
    }

    #[rstest]
    #[case::unknown_image("zookeeper")]
    #[case::unknown_version("kafka=1.0.0")]
    fn reject_unknown_selectors(expectations: ExpectationsFile, #[case] selector: &str) {
        let repository = repository(&[("kafka", &["3.9.2", "4.3.1"])]);
        let result = expectations.select_targets(
            repository.path(),
            &[selector.to_owned()],
            &["amd64".to_owned()],
        );
        assert!(result.is_err(), "{selector:?} should be rejected");
    }

    #[rstest]
    fn report_new_product_and_removed_version(expectations: ExpectationsFile) {
        let repository = repository(&[("kafka", &["4.3.1"]), ("new-product", &["1.0.0"])]);

        let (_, problems) = expectations
            .select_targets(repository.path(), &[], &["amd64".to_owned()])
            .expect("the targets should be selected");

        assert_eq!(
            problems,
            [
                "[products.kafka.versions] has expectations for 3.9.2, which boil does not build",
                "new-product is built by boil but is neither in [products] nor in `unchecked`",
            ]
        );
    }
}
