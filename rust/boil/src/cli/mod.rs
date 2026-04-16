use std::{path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};
use semver::Version;
use snafu::{ResultExt, Snafu, ensure};

mod build;
mod completions;
mod image;

pub use build::*;
pub use completions::*;
pub use image::*;
use url::Host;

#[derive(Debug, Snafu)]
pub enum ParseImageVersionError {
    #[snafu(display("failed to parse semantic version"))]
    ParseVersion { source: semver::Error },

    #[snafu(display("semantic version must not contain build metadata"))]
    ContainsBuildMetadata,
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

    pub(super) fn default_image_version() -> Version {
        "0.0.0-dev"
            .parse()
            .expect("static string must be a valid SemVer")
    }

    pub(super) fn default_registry() -> HostPort {
        HostPort {
            host: Host::Domain(String::from("oci.stackable.tech")),
            port: None,
        }
    }

    pub(super) fn parse_image_version(input: &str) -> Result<Version, ParseImageVersionError> {
        let version = Version::from_str(input).context(ParseVersionSnafu)?;
        ensure!(version.build.is_empty(), ContainsBuildMetadataSnafu);

        Ok(version)
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
