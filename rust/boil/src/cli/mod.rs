use std::{path::PathBuf, sync::LazyLock};

use clap::{Parser, Subcommand};
use regex::Regex;
use snafu::Snafu;

mod build;
mod completions;
mod image;

pub use build::*;
pub use completions::*;
pub use image::*;
use url::Host;

// This is derived from the general rule where the length of the tag can be up to 128 chars
// See: https://github.com/opencontainers/distribution-spec/blob/main/spec.md
// But that checking needs to be at a higher layer.
static VALID_IMAGE_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_][a-zA-Z0-9_.-]+$").expect("regex is valid"));

#[derive(Debug, Snafu)]
pub enum ParseImageVersionError {
    #[snafu(display("invalid image tag characters for {version:?}"))]
    ParseVersion { version: String },
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to the configuration file.
    #[arg(short, long = "configuration", global = true, default_value_os_t = Self::default_config_path())]
    pub config_path: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    fn default_config_path() -> PathBuf {
        PathBuf::from("./boil.toml")
    }

    pub(super) fn default_image_version() -> String {
        "0.0.0-dev".to_owned()
    }

    pub(super) fn default_registry() -> HostPort {
        HostPort {
            host: Host::Domain(String::from("oci.stackable.tech")),
            port: None,
        }
    }

    /// Ensure that the given version will be valid for use in the image tag
    pub(super) fn parse_image_version(version: &str) -> Result<String, ParseImageVersionError> {
        if !VALID_IMAGE_TAG.is_match(version) {
            return ParseVersionSnafu { version }.fail();
        }

        Ok(version.to_owned())
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Build one or more images.
    ///
    /// Requires docker with the buildx extension.
    #[command(alias = "some-chicken")]
    Build(Box<BuildArguments>),

    /// Display various structured outputs in JSON format.
    Image(ImageArguments),

    /// Alias for `image list`.
    Images(ImageListArguments),

    /// Generate shell completions.
    Completions(CompletionsArguments),
}
