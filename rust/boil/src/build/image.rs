use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::Deserialize;
use snafu::{ResultExt as _, Snafu};

use crate::{IfContext, build::docker::BuildArguments};

#[derive(Debug, Snafu)]
pub enum ParseImageError {
    #[snafu(display("encountered invalid format, expected name[=version,...]"))]
    InvalidFormat,
}

#[derive(Clone, Debug)]
pub struct Image {
    pub name: String,
    pub versions: Vec<String>,
}

impl FromStr for Image {
    type Err = ParseImageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('=').collect();

        match parts.len() {
            1 => Ok(Self::new_unversioned(parts[0].to_owned())),
            2 => {
                let versions: Vec<_> = parts[1].split(',').map(ToOwned::to_owned).collect();
                Ok(Self::new(parts[0].to_owned(), versions))
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
    pub versions: ImageVersions,
}

impl ImageConfig {
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
