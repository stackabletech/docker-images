use clap::{Args, Subcommand, ValueEnum};
use semver::Version;

use crate::{cli::Cli, core::image::ImageSelector};

#[derive(Debug, Args)]
pub struct ImageArguments {
    #[command(subcommand)]
    pub command: ImageCommand,
}

#[derive(Debug, Subcommand)]
pub enum ImageCommand {
    /// Lists images known by boil with all available versions.
    List(ImageListArguments),

    /// Checks if all images known by boil are available in the specified registries.
    ///
    /// Access tokens must be provided with the following name: `BOIL_REGISTRY_TOKEN_<REGISTRY_URI>`.
    Check(ImageCheckArguments),
}

#[derive(Debug, Args)]
pub struct ImageListArguments {
    /// Optionally specify one or more images to display.
    pub image: Vec<ImageSelector>,

    /// Pretty print the structured output.
    #[arg(long, value_enum, default_value_t = Pretty::default())]
    pub pretty: Pretty,
}

#[derive(Debug, Args)]
pub struct ImageCheckArguments {
    /// Optionally specify one or more images to check. Checks all images by default.
    pub image: Vec<ImageSelector>,

    // NOTE (@Techassi): Should this maybe be renamed to vendor_version?
    /// The image version being built.
    #[arg(
        short, long,
        value_parser = Cli::parse_image_version,
        default_value_t = Cli::default_image_version(),
        help_heading = "Image Options"
    )]
    pub image_version: Version,
}

// #[derive(Clone, Debug, Default, strum::Display, strum::EnumString)]
#[derive(Clone, Debug, Default, ValueEnum)]
pub enum Pretty {
    #[default]
    Auto,
    Always,
    Never,
}
