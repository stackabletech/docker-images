use clap::{Args, ValueEnum};

use crate::build::image::Image;

#[derive(Debug, Args)]
pub struct ShowImagesArguments {
    /// Optionally specify one or more images to display.
    pub image: Vec<Image>,

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
