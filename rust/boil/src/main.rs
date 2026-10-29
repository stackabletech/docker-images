use clap::Parser;
use snafu::{ResultExt, Snafu};

use crate::{
    cli::{Cli, Command, ImageCommand, ToolsCommand},
    config::Config,
};

mod cli;
mod cmd;
mod config;
mod constants;
mod core;
mod models;
mod utils;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("failed to run build command"))]
    Build { source: cmd::build::Error },

    #[snafu(display("failed to run image command"))]
    Image { source: cmd::image::Error },

    #[snafu(display("failed to read config"))]
    ReadConfig { source: config::ConfigError },
}

#[tokio::main(flavor = "current_thread")]
#[snafu::report]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::Build(arguments) => {
            let config = Config::from_file(&cli.config_path).context(ReadConfigSnafu)?;
            cmd::build::run_command(arguments, config).context(BuildSnafu)
        }
        Command::Image(arguments) => match arguments.command {
            ImageCommand::List(arguments) => cmd::image::list_images(arguments).context(ImageSnafu),
            ImageCommand::Check(arguments) => {
                let config = Config::from_file(&cli.config_path).context(ReadConfigSnafu)?;
                cmd::image::check_images(arguments, config)
                    .await
                    .context(ImageSnafu)
            }
            ImageCommand::Size(arguments) => {
                let config = Config::from_file(&cli.config_path).context(ReadConfigSnafu)?;
                cmd::image::calculate_size(arguments, config)
                    .await
                    .context(ImageSnafu)
            }
        },
        Command::Images(arguments) => cmd::image::list_images(arguments).context(ImageSnafu),
        Command::Tools(arguments) => match arguments.command {
            ToolsCommand::FloatingTag(arguments) => {
                cmd::tools::floating_tag(arguments);
                Ok(())
            }
        },
        Command::Completions(arguments) => {
            cmd::completions::run_command(arguments);
            Ok(())
        }
    }
}
