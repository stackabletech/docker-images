use clap::{Args, Subcommand, ValueEnum};

use crate::core::image::ImageSelector;

#[derive(Debug, Args)]
pub struct ImageArguments {
    #[command(subcommand)]
    pub command: ImageCommand,
}

#[derive(Debug, Subcommand)]
pub enum ImageCommand {
    List(ImageListArguments),
}

#[derive(Debug, Args)]
pub struct ImageListArguments {
    /// Optionally specify one or more images to display.
    pub image: Vec<ImageSelector>,

    /// Pretty print the structured output.
    #[arg(long, value_enum, default_value_t = Pretty::default())]
    pub pretty: Pretty,
}

// #[derive(Clone, Debug, Default, strum::Display, strum::EnumString)]
#[derive(Clone, Debug, Default, ValueEnum)]
pub enum Pretty {
    #[default]
    Auto,
    Always,
    Never,
}
