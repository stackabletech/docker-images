use clap::Parser;
use snafu::{ResultExt, Snafu};

use crate::{
    cli::{Cli, Command, ImageCommand},
    config::Config,
};

mod cli;
mod cmd;
mod config;
mod constants;
mod core;
mod models;
mod utils;

/// This trait extends functionailty provided by [`snafu`].
///
/// [`snafu`] already provides various ways to extend [`Result`]s with additional context-sensitive
/// information. This trait allows calling `if_context` on any type, which runs a predicate to
/// determine if an error with the provided context should be returned.
///
/// This trait can be thought of as a combination of [`snafu::ensure!`] and returning [`Ok`]
/// afterwards.
pub trait IfContext: Sized {
    /// Runs `predicate` and returns [`Ok`] if `true` or [`Err`] (with data from `context`) otherwise.
    fn if_context<P, C, E>(self, predicate: P, context: C) -> Result<Self, E>
    where
        P: Fn(&Self) -> bool,
        C: snafu::IntoError<E, Source = snafu::NoneError>,
        E: std::error::Error + snafu::ErrorCompat;
}

impl<T> IfContext for T {
    fn if_context<P, C, E>(self, predicate: P, context: C) -> Result<Self, E>
    where
        P: Fn(&Self) -> bool,
        C: snafu::IntoError<E, Source = snafu::NoneError>,
        E: std::error::Error + snafu::ErrorCompat,
    {
        match predicate(&self) {
            true => Ok(self),
            false => Err(context.into_error(snafu::NoneError)),
        }
    }
}

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
        },
        Command::Images(arguments) => cmd::image::list_images(arguments).context(ImageSnafu),
        Command::Completions(arguments) => {
            cmd::completions::run_command(arguments);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO (@Techassi): These tests are currently commented out because rstest
    // contains a bug related to the Rust's core library and the local core
    // module. It is fixed upstream, but not released yet.
    // Upstream fix PR: https://github.com/la10736/rstest/pull/336
    // use rstest::rstest;

    // use super::*;

    // #[rstest]
    // #[case("25.11.0-rc.1+arm64", "25.11.0-rc.1")]
    // #[case("25.11.0-rc.1", "25.11.0-rc.1")]
    // #[case("25.11.0-rc1", "25.11.0-rc1")]
    // #[case("0.0.0-dev", "0.0.0-dev")]
    // #[case("25.11.0", "25.11.0")]
    // #[case("0.0.0", "0.0.0")]
    // fn version_ext_base_prerelease(#[case] input: &str, #[case] expected: &str) {
    //     let version: Version = input.parse().expect("must be a valid semantic version");
    //     assert_eq!(version.base_prerelease(), expected);
    // }

    // #[rstest]
    // #[case("25.11.0-rc.1+arm64", "25.11.0")]
    // #[case("25.11.0-rc.1", "25.11.0")]
    // #[case("25.11.0-rc1", "25.11.0")]
    // #[case("0.0.0-dev", "0.0.0")]
    // #[case("25.11.0", "25.11.0")]
    // #[case("0.0.0", "0.0.0")]
    // fn version_ext_base(#[case] input: &str, #[case] expected: &str) {
    //     let version: Version = input.parse().expect("must be a valid semantic version");
    //     assert_eq!(version.base(), expected);
    // }
}
