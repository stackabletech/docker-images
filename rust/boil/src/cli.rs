use std::{path::PathBuf, str::FromStr};

use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;
use semver::Version;
use snafu::{ResultExt, Snafu, ensure};

use crate::build::cli::BuildArguments;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to the configuration file.
    #[arg(short = 'c', long = "configuration", default_value_os_t = Self::default_config_path())]
    pub config_path: PathBuf,

    /// Path to the OpenShift configuration file.
    #[arg(long, default_value_os_t = Self::default_openshift_config_path())]
    pub openshift_config_path: PathBuf,

    #[arg(short, long, default_value_os_t = Self::default_base_path())]
    pub base_path: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    fn default_config_path() -> PathBuf {
        PathBuf::from("./boil.toml")
    }

    fn default_openshift_config_path() -> PathBuf {
        PathBuf::from("./openshift.toml")
    }

    fn default_base_path() -> PathBuf {
        PathBuf::from(".")
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Build one or more product images.
    ///
    /// Requires docker with the buildx extension.
    #[command(alias = "some-chicken")]
    Build(BuildArguments),

    /// Display various structured outputs in JSON format.
    Show(ShowArguments),

    /// Generate shell completions.
    Completions(CompletionsArguments),
}

#[derive(Debug, Args)]
pub struct ShowArguments {
    #[command(subcommand)]
    pub commands: ShowCommand,
}

#[derive(Debug, Subcommand)]
pub enum ShowCommand {
    Images,
    Tree,
}

#[derive(Debug, Args)]
pub struct CompletionsArguments {
    /// Shell to generate completions for.
    pub shell: Shell,
}

#[derive(Debug, Snafu)]
pub enum ParseImageVersionError {
    #[snafu(display("failed to parse semantic version"))]
    ParseVersion { source: semver::Error },

    #[snafu(display("semantic version must not contain build metadata"))]
    ContainsBuildMetadata,
}

pub fn parse_image_version(input: &str) -> Result<Version, ParseImageVersionError> {
    let version = Version::from_str(input).context(ParseVersionSnafu)?;
    ensure!(version.build.is_empty(), ContainsBuildMetadataSnafu);

    Ok(version)
}
