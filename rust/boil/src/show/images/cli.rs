use clap::Args;

use crate::build::image::Image;

#[derive(Debug, Args)]
pub struct ShowImagesArguments {
    /// Optionally specify one or more images to display.
    pub image: Vec<Image>,

    /// Pretty print the structured output.
    #[arg(long)]
    pub pretty: bool,
}
