use std::path::Path;

use serde::Deserialize;
use snafu::{ResultExt, Snafu};

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
}

impl Config {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let contents = std::fs::read_to_string(path).context(ReadFileSnafu)?;
        toml::from_str(&contents).context(DeserializeSnafu)
    }
}
