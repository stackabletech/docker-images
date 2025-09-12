use clap::Parser;
use semver::Version;
use snafu::{ResultExt, Snafu};

use crate::{
    cli::{Cli, Command},
    config::Config,
    show::ShowCommand,
};

// Common modules
mod cli;
mod config;
mod utils;

// Command modules
mod build;
mod completions;
mod show;

pub trait IfContext: Sized {
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

pub trait VersionExt {
    /// Returns the base of a [`Version`] as a string, eg. `1.2.3`.
    fn base(&self) -> String;

    /// Returns the base and prerelease of a [`Version`] as a string, eg. `1.2.3-rc.1`.
    fn base_prerelease(&self) -> String;
}

impl VersionExt for Version {
    fn base(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    fn base_prerelease(&self) -> String {
        let mut base = self.base();
        base.push('-');
        base.push_str(&self.pre);
        base
    }
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("failed to run build command"))]
    Build { source: build::Error },

    #[snafu(display("failed to run show command"))]
    Show { source: show::images::Error },

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
            build::run_command(arguments, config).context(BuildSnafu)
        }
        Command::Show(arguments) => match arguments.commands {
            ShowCommand::Images(arguments) => {
                show::images::run_command(arguments).context(ShowSnafu)
            }
        },
        Command::Completions(arguments) => {
            completions::run_command(arguments);
            Ok(())
        }
    }
}
