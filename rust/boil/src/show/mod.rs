use clap::{Args, Subcommand};

use crate::show::images::cli::ShowImagesArguments;

pub mod images;

#[derive(Debug, Args)]
pub struct ShowArguments {
    #[command(subcommand)]
    pub commands: ShowCommand,
}

#[derive(Debug, Subcommand)]
pub enum ShowCommand {
    Images(ShowImagesArguments),
    Tree,
}
