use clap::{Args, Subcommand, ValueEnum};

use crate::{
    cli::Cli,
    core::{
        image::ImageSelector,
        platform::{Architecture, TargetPlatform},
    },
};

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

    /// Calculates the size of images known by boil.
    Size(ImageSizeArguments),
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
    /// The image version to check.
    #[arg(
        short, long,
        value_parser = Cli::parse_image_version,
        default_value_t = Cli::default_image_version(),
        help_heading = "Image Options"
    )]
    pub image_version: String,
}

#[derive(Debug, Args)]
pub struct ImageSizeArguments {
    /// Optionally specify one or more images to check. Checks all images by default.
    pub image: Vec<ImageSelector>,

    // NOTE (@Techassi): Should this maybe be renamed to vendor_version?
    /// The image version to use.
    #[arg(
        short, long,
        value_parser = Cli::parse_image_version,
        default_value_t = Cli::default_image_version(),
        help_heading = "Image Options"
    )]
    pub image_version: String,

    /// Target platform of the image.
    #[arg(
        short, long,
        short_alias = 'a', alias = "architecture",
        default_value_t = Self::default_architecture(),
        help_heading = "Image Options"
    )]
    pub target_platform: TargetPlatform,

    /// Pretty print the structured output.
    #[arg(long, value_enum, default_value_t = Pretty::default(), help_heading = "Output Options")]
    pub pretty: Pretty,

    #[arg(short, long, value_enum, default_value_t = Format::default(), help_heading = "Output Options")]
    pub format: Format,
}

impl ImageSizeArguments {
    // TODO: Auto-detect this
    fn default_architecture() -> TargetPlatform {
        TargetPlatform::Linux(Architecture::Amd64)
    }
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum Pretty {
    #[default]
    Auto,
    Always,
    Never,
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum Format {
    #[default]
    Plain,
    Json,
}
