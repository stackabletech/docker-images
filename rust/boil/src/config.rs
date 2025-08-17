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
pub struct Metadata {
    pub documentation: Url,
    pub licenses: String,
    pub authors: String,
    pub vendor: String,
    pub source: Url,
}

#[derive(Debug, Deserialize)]
pub struct DockerConfig {}
