use std::path::Path;

use serde::Deserialize;
use snafu::{ResultExt, Snafu};
use url::Url;

use crate::build::docker::BuildArguments;

#[derive(Debug, Snafu)]
pub enum ConfigError {
    ReadFile { source: std::io::Error },

    Deserialize { source: toml::de::Error },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub build_arguments: BuildArguments,
    pub metadata: Metadata,
}

impl Config {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let contents = std::fs::read_to_string(path).context(ReadFileSnafu)?;
        toml::from_str(&contents).context(DeserializeSnafu)
    }
}

// NOTE (@Techassi): Think about if these metadata fields should be required or optional. If they
// are optional, the appropriate annotations are only emitted if set.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    /// The URL to the documentation page.
    pub documentation: Option<Url>,

    /// One ore more licenses used for images using the SPDX format.
    pub licenses: Option<String>,

    /// One or more authors of images.
    ///
    /// It is recommended to use the "NAME <EMAIL>" format.
    pub authors: Option<String>,

    /// The vendor who builds the images.
    pub vendor: Option<String>,

    /// The vendor prefix used in the image (index) manifest tag.
    ///
    /// Defaults to an empty string.
    #[serde(default)]
    pub vendor_tag_prefix: String,

    /// The version control source of the images.
    pub source: Option<Url>,
}
