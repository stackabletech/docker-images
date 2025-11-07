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

pub trait VersionExt {
    /// Returns the base of a [`Version`] as a string, eg. `1.2.3`.
    fn base(&self) -> String;

    /// Returns the base and prerelease of a [`Version`] as a string, eg. `1.2.3-rc.1`.
    fn base_prerelease(&self) -> String;
}

impl VersionExt for Version {
    fn base(&self) -> String {
        let Self {
            major,
            minor,
            patch,
            ..
        } = self;

        format!("{major}.{minor}.{patch}")
    }

    fn base_prerelease(&self) -> String {
        let mut base = self.base();

        // Well, that was a big doozy, ruined the whole release...
        if !self.pre.is_empty() {
            base.push('-');
            base.push_str(&self.pre);
        }

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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("25.11.0-rc.1+arm64", "25.11.0-rc.1")]
    #[case("25.11.0-rc.1", "25.11.0-rc.1")]
    #[case("25.11.0-rc1", "25.11.0-rc1")]
    #[case("0.0.0-dev", "0.0.0-dev")]
    #[case("25.11.0", "25.11.0")]
    #[case("0.0.0", "0.0.0")]
    fn version_ext_base_prerelease(#[case] input: &str, #[case] expected: &str) {
        let version: Version = input.parse().expect("must be a valid semantic version");
        assert_eq!(version.base_prerelease(), expected);
    }

    #[rstest]
    #[case("25.11.0-rc.1+arm64", "25.11.0")]
    #[case("25.11.0-rc.1", "25.11.0")]
    #[case("25.11.0-rc1", "25.11.0")]
    #[case("0.0.0-dev", "0.0.0")]
    #[case("25.11.0", "25.11.0")]
    #[case("0.0.0", "0.0.0")]
    fn version_ext_base(#[case] input: &str, #[case] expected: &str) {
        let version: Version = input.parse().expect("must be a valid semantic version");
        assert_eq!(version.base(), expected);
    }
}
