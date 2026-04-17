use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::Deserialize;
use snafu::{ResultExt as _, Snafu, ensure};

use crate::core::docker;

#[derive(Debug, PartialEq, Snafu)]
pub enum ParseImageSelectorError {
    #[snafu(display("input must not be empty"))]
    EmptyInput,

    #[snafu(display("encountered invalid format, expected name[=version,...]"))]
    InvalidFormat,

    #[snafu(display("the path contains unsupported characters: '.' or '~'"))]
    UnsupportedChars,

    #[snafu(display("absolute paths are not supported"))]
    AbsolutePath,
}

/// Represents an image selector (`<NAME>=<VERSION>[,<VERSION>...]`) used as a CLI argument.
#[derive(Clone, Debug)]
pub struct ImageSelector {
    pub name: String,
    pub versions: Vec<String>,
}

impl FromStr for ImageSelector {
    type Err = ParseImageSelectorError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Get rid of any leading and traling whitespace
        let input = input.trim();
        ensure!(!input.is_empty(), EmptyInputSnafu);

        let parts: Vec<_> = input.split('=').collect();

        // Ensure that the path/image name is not empty, doesn't contain '~', and is not abolute.
        ensure!(!parts[0].is_empty(), InvalidFormatSnafu);
        ensure!(!parts[0].contains('~'), UnsupportedCharsSnafu);
        ensure!(!parts[0].starts_with('/'), AbsolutePathSnafu);

        // Get rid of a leading ./ from the image name. This would need to be replaced, because
        // Docker doesn't allow dots in various places (like target names). Additionally, it would
        // clutter the different names. The same applies for a trailing slash.
        let image_name = parts[0]
            .trim_start_matches("./")
            .trim_end_matches('/')
            .to_owned();

        match parts.len() {
            1 => Ok(Self::new_unversioned(image_name)),
            2 => {
                // Ensure that the version part is not empty
                ensure!(!parts[1].is_empty(), InvalidFormatSnafu);

                let versions: Vec<_> = parts[1].split(',').map(ToOwned::to_owned).collect();
                Ok(Self::new(image_name, versions))
            }
            _ => InvalidFormatSnafu.fail(),
        }
    }
}

impl Display for ImageSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.versions.is_empty() {
            f.write_str(&self.name)
        } else {
            write!(
                f,
                "{name}={versions}",
                name = self.name,
                versions = self.versions.join(",")
            )
        }
    }
}

impl ImageSelector {
    fn new(name: String, versions: Vec<String>) -> Self {
        Self { name, versions }
    }

    fn new_unversioned(name: String) -> Self {
        Self {
            name,
            versions: vec![],
        }
    }
}

#[derive(Debug, Snafu)]
pub enum ImageConfigError {
    #[snafu(display("failed to read config file at {path}", path = path.display()))]
    ReadFile {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to deserialize config file from TOML"))]
    Deserialize { source: toml::de::Error },

    #[snafu(display("provided filter version yielded empty list"))]
    EmptyFilter,
}

#[derive(Debug, Deserialize)]
pub struct ImageConfig {
    #[serde(default)]
    pub metadata: ImageMetadata,

    pub versions: ImageVersions,
}

impl ImageConfig {
    /// This glob pattern matches all (deeply nested) image configs.
    pub const ALL_CONFIGS_GLOB_PATTERN: &str = "**/boil-config.toml";
    /// The default image config file name.
    pub const DEFAULT_FILE_NAME: &str = "boil-config.toml";
    /// This glob pattern matches all (top-level) image configs.
    pub const FLAT_CONFIG_GLOB_PATTERN: &str = "*/boil-config.toml";

    /// This function removes versions in the config filtered out by `versions`.
    pub fn filter_by_version<V>(&mut self, versions: &[V]) -> Result<(), ImageConfigError>
    where
        V: AsRef<str> + PartialEq,
    {
        self.versions.retain(|image_version, _| {
            versions.is_empty() || versions.iter().any(|v| v.as_ref() == image_version)
        });

        ensure!(!self.versions.is_empty(), EmptyFilterSnafu);
        Ok(())
    }
}

impl ImageConfig {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ImageConfigError> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path).with_context(|_| ReadFileSnafu { path })?;
        toml::from_str(&contents).context(DeserializeSnafu)
    }
}

#[derive(Debug, Deserialize)]
pub struct ImageVersions(BTreeMap<String, ImageVersionOptions>);

impl Deref for ImageVersions {
    type Target = BTreeMap<String, ImageVersionOptions>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ImageVersions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for ImageVersions {
    type IntoIter = std::collections::btree_map::IntoIter<String, ImageVersionOptions>;
    type Item = (String, ImageVersionOptions);

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImageVersionOptions {
    #[serde(default)]
    pub local_images: BTreeMap<String, String>,

    // NOTE (@Techassi): Potentially add a dependencies field here which will be automatically be
    // suffixed with _VERSION.
    #[serde(default)]
    pub build_arguments: docker::BuildArguments,

    /// A custom path to a Dockerfile/Containerfile for a particular version of an image.
    ///
    /// This is usefull for cases where the same image is being built differently depending on it's
    /// version and it is too difficult/messy to do it the same Dockerfile/Containerfile.
    #[serde(alias = "containerfile")]
    pub dockerfile: Option<PathBuf>,
}

#[derive(Debug, Default, Deserialize)]
pub struct ImageMetadata {
    /// A map of registries an image is published to and various options configurable for each one.
    #[serde(default)]
    pub registries: BTreeMap<String, RegistryOptions>,
}

#[derive(Debug, Deserialize)]
pub struct RegistryOptions {
    pub namespace: String,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("my/image/in.a/folder/with/name=1.2.3-rc.1,4.5.6-rc.2", &["1.2.3-rc.1", "4.5.6-rc.2"])]
    #[case("my/image/in.a/folder/with/name=1.2.3-rc.1", &["1.2.3-rc.1"])]
    #[case("my.image.in.a.folder.with/name=1.2.3", &["1.2.3"])]
    #[case("my/image/in.a/folder/with/name", &[])]
    #[case("my.image.in.a.folder.with/name", &[])]
    #[case("my/image/with/name=1.2.3", &["1.2.3"])]
    #[case("my/image/with/name", &[])]
    #[case("name=1.2.3", &["1.2.3"])]
    #[case("name", &[])]
    fn valid(#[case] input: &str, #[case] expected_versions: &[&str]) {
        let ImageSelector { versions, .. } =
            ImageSelector::from_str(input).expect("must be a valid image");
        assert_eq!(versions, expected_versions);
    }

    #[rstest]
    #[case(
        "double/equal/image=1.2.3=4.5.6",
        ParseImageSelectorError::InvalidFormat
    )]
    #[case("~/image/folder/with/tilde", ParseImageSelectorError::UnsupportedChars)]
    #[case("/absolute/image/folder", ParseImageSelectorError::AbsolutePath)]
    #[case("empty/version/image=", ParseImageSelectorError::InvalidFormat)]
    #[case("   ", ParseImageSelectorError::EmptyInput)]
    #[case("", ParseImageSelectorError::EmptyInput)]
    fn invalid(#[case] input: &str, #[case] expected_error: ParseImageSelectorError) {
        let error = ImageSelector::from_str(input).expect_err("invalid image must not parse");
        assert_eq!(error, expected_error);
    }
}
