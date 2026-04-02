use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::Deserialize;
use snafu::{ResultExt as _, Snafu, ensure};

use crate::{IfContext, build::docker::BuildArguments};

#[derive(Debug, PartialEq, Snafu)]
pub enum ParseImageError {
    #[snafu(display("input must not be empty"))]
    EmptyInput,

    #[snafu(display("encountered invalid format, expected name[=version,...]"))]
    InvalidFormat,

    #[snafu(display("the path contains unsupported characters: '.' or '~'"))]
    UnsupportedChars,

    #[snafu(display("absolute paths are not supported"))]
    AbsolutePath,
}

#[derive(Clone, Debug)]
pub struct Image {
    pub name: String,
    pub versions: Vec<String>,
}

impl FromStr for Image {
    type Err = ParseImageError;

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

impl Display for Image {
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

impl Image {
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
    // TODO (@Techassi): Eventually support this
    // #[serde(default)]
    // pub metadata: ImageMetadata,
    pub versions: ImageVersions,
}

impl ImageConfig {
    /// This glob pattern matches all (deeply nested) image configs.
    pub const ALL_CONFIGS_GLOB_PATTERN: &str = "**/boil-config.toml";
    /// The default image config file name.
    pub const DEFAULT_FILE_NAME: &str = "boil-config.toml";

    pub fn filter_by_version<V>(
        self,
        versions: &[V],
    ) -> Result<Vec<VersionOptionsPair>, ImageConfigError>
    where
        V: AsRef<str> + PartialEq,
    {
        let versions: Vec<_> = self
            .pairs()
            .filter(|(image_version, _)| {
                versions.is_empty() || versions.iter().any(|v| v.as_ref() == image_version)
            })
            .map(Into::into)
            .collect();

        versions.if_context(|v| !v.is_empty(), EmptyFilterSnafu)
    }

    pub fn all(self) -> Vec<VersionOptionsPair> {
        self.pairs().map(Into::into).collect()
    }

    fn pairs(self) -> impl Iterator<Item = (String, ImageOptions)> {
        self.versions.0.into_iter()
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
pub struct ImageVersions(BTreeMap<String, ImageOptions>);

impl Deref for ImageVersions {
    type Target = BTreeMap<String, ImageOptions>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImageOptions {
    #[serde(default)]
    pub local_images: BTreeMap<String, String>,

    // NOTE (@Techassi): Potentially add a dependencies field here which will be automatically be
    // suffixed with _VERSION.
    #[serde(default)]
    pub build_arguments: BuildArguments,

    /// A custom path to a Dockerfile/Containerfile for a particular version of an image.
    ///
    /// This is usefull for cases where the same image is being built differently depending on it's
    /// version and it is too difficult/messy to do it the same Dockerfile/Containerfile.
    #[serde(alias = "containerfile")]
    pub dockerfile: Option<PathBuf>,
}

#[derive(Debug)]
pub struct VersionOptionsPair {
    pub version: String,
    pub options: ImageOptions,
}

impl From<(String, ImageOptions)> for VersionOptionsPair {
    fn from(value: (String, ImageOptions)) -> Self {
        VersionOptionsPair {
            version: value.0,
            options: value.1,
        }
    }
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
        let Image { versions, .. } = Image::from_str(input).expect("must be a valid image");
        assert_eq!(versions, expected_versions);
    }

    #[rstest]
    #[case("double/equal/image=1.2.3=4.5.6", ParseImageError::InvalidFormat)]
    #[case("~/image/folder/with/tilde", ParseImageError::UnsupportedChars)]
    #[case("/absolute/image/folder", ParseImageError::AbsolutePath)]
    #[case("empty/version/image=", ParseImageError::InvalidFormat)]
    #[case("   ", ParseImageError::EmptyInput)]
    #[case("", ParseImageError::EmptyInput)]
    fn invalid(#[case] input: &str, #[case] expected_error: ParseImageError) {
        let error = Image::from_str(input).expect_err("invalid image must not parse");
        assert_eq!(error, expected_error);
    }
}
