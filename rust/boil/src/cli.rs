use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{build::cli::BuildArguments, completions::CompletionsArguments, show::ShowArguments};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to the configuration file.
    #[arg(short, long = "configuration", global = true, default_value_os_t = Self::default_config_path())]
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
