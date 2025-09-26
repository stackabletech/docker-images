use std::{
    fmt::{Debug, Display},
    path::PathBuf,
    str::FromStr,
};

use clap::{Args, ValueHint, value_parser};
use semver::Version;
use snafu::{ResultExt, Snafu, ensure};
use url::Host;

use crate::build::{
    docker::BuildArgument,
    image::Image,
    platform::{Architecture, TargetPlatform},
};

#[derive(Debug, Args)]
pub struct BuildArguments {
    /// The image(s) which should be build. The format is name[=version,...].
    #[arg(help_heading = "Image Options", required = true)]
    pub images: Vec<Image>,

    // The action currently does the wrong thing here. It includes the
    // architecture even though it should come from the --target-platform arg.
    // The release arg is NOT needed, because this version IS the release version.
    /// The image version being built.
    #[arg(
        short, long,
        value_parser = parse_image_version,
        default_value_t = Self::default_image_version(),
        help_heading = "Image Options"
    )]
    pub image_version: Version,

    /// Target platform of the image.
    #[arg(
        short, long,
        short_alias = 'a', alias = "architecture",
        default_value_t = Self::default_architecture(),
        help_heading = "Image Options"
    )]
    pub target_platform: TargetPlatform,

    /// Image registry used in image manifests, URIs, and tags.
    /// The format is host[:port].
    #[arg(
        short, long,
        default_value_t = HostPort::localhost(),
        value_hint = ValueHint::Hostname,
        help_heading = "Registry Options"
    )]
    pub registry: HostPort,

    /// The namespace within the given registry.
    #[arg(
        short = 'n',
        long = "registry-namespace",
        alias = "organization",
        default_value = "sdp",
        help_heading = "Registry Options"
    )]
    pub registry_namespace: String,

    /// Use 'localhost' as the registry instead of <REGISTRY> to avoid any accidental interactions
    /// with remote registries.
    ///
    /// This is especially useful in CI, which can re-tag the image before pushing it.
    #[arg(long, help_heading = "Registry Options")]
    pub use_localhost_registry: bool,

    /// Override the target containerfile used, points to <IMAGE>/<TARGET_CONTAINERFILE>.
    #[arg(
        long,
        default_value_os_t = Self::default_target_containerfile(),
        value_hint = ValueHint::FilePath,
        help_heading = "Build Options"
    )]
    pub target_containerfile: PathBuf,

    /// Override build arguments, in key=value format. The key is case insensitive. This argument
    /// can be supplied multiple times.
    #[arg(
        long = "build-argument",
        alias = "build-arg",
        value_name = "BUILD_ARGUMENT",
        help_heading = "Build Options"
    )]
    pub build_arguments: Vec<BuildArgument>,

    /// Load and override build arguments, in key=value format, each separated by a newline from the
    /// specified file.
    #[arg(long, alias = "build-args-file", help_heading = "Build Options")]
    pub build_arguments_file: Option<PathBuf>,

    /// Write target image tags to <EXPORT_FILE>. Useful for signing or other follow-up CI steps.
    #[arg(
        long,
        alias = "export-tags-file",
        help_heading = "Build Options",
        value_name = "FILE",
        value_hint = ValueHint::FilePath,
        value_parser = value_parser!(PathBuf),
        default_missing_value = "boil-target-tags",
        num_args(0..=1)
    )]
    pub write_image_manifest_uris: Option<PathBuf>,

    /// Strips the architecture from the image (index) manifest tag.
    #[arg(long, help_heading = "Build Options")]
    pub strip_architecture: bool,

    /// Loads the image into the local image store.
    #[arg(long, help_heading = "Build Options")]
    pub load: bool,

    /// Dry run. This does not build the image(s) but instead prints out the bakefile.
    #[arg(short, long, alias = "dry")]
    pub dry_run: bool,
}

impl BuildArguments {
    fn default_image_version() -> Version {
        "0.0.0-dev".parse().expect("must be a valid SemVer")
    }

    // TODO: Auto-detect this
    fn default_architecture() -> TargetPlatform {
        TargetPlatform::Linux(Architecture::Amd64)
    }

    fn default_target_containerfile() -> PathBuf {
        PathBuf::from("Dockerfile")
    }
}

#[derive(Debug, Snafu)]
pub enum ParseImageVersionError {
    #[snafu(display("failed to parse semantic version"))]
    ParseVersion { source: semver::Error },

    #[snafu(display("semantic version must not contain build metadata"))]
    ContainsBuildMetadata,
}

pub fn parse_image_version(input: &str) -> Result<Version, ParseImageVersionError> {
    let version = Version::from_str(input).context(ParseVersionSnafu)?;
    ensure!(version.build.is_empty(), ContainsBuildMetadataSnafu);

    Ok(version)
}

#[derive(Debug, PartialEq, Snafu)]
pub enum ParseHostPortError {
    #[snafu(display("unexpected empty input"))]
    EmptyInput,

    #[snafu(display("invalid format, expected host[:port]"))]
    InvalidFormat,

    #[snafu(display("failed to parse host"))]
    InvalidHost { source: url::ParseError },

    #[snafu(display("failed to parse port"))]
    InvalidPort { source: std::num::ParseIntError },
}

#[derive(Clone, Debug)]
pub struct HostPort {
    pub host: Host,
    pub port: Option<u16>,
}

impl Display for HostPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.port {
            Some(port) => write!(f, "{host}:{port}", host = self.host),
            None => Display::fmt(&self.host, f),
        }
    }
}

impl FromStr for HostPort {
    type Err = ParseHostPortError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        ensure!(!input.is_empty(), EmptyInputSnafu);

        let parts: Vec<_> = input.split(':').collect();

        match parts.len() {
            1 => {
                let host = Host::parse(parts[0]).context(InvalidHostSnafu)?;
                Ok(Self { host, port: None })
            }
            2 => {
                let host = Host::parse(parts[0]).context(InvalidHostSnafu)?;
                let port = u16::from_str(parts[1]).context(InvalidPortSnafu)?;

                Ok(Self {
                    host,
                    port: Some(port),
                })
            }
            _ => InvalidFormatSnafu.fail(),
        }
    }
}

impl HostPort {
    pub fn localhost() -> Self {
        HostPort {
            host: Host::Domain(String::from("localhost")),
            port: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use url::ParseError;

    use super::*;

    #[rstest]
    #[case("registry.example.org:65535")]
    #[case("registry.example.org:8080")]
    #[case("registry.example.org")]
    #[case("example.org:8080")]
    #[case("localhost:8080")]
    #[case("example.org")]
    #[case("localhost")]
    fn valid_host_port(#[case] input: &str) {
        let host_port = HostPort::from_str(input).expect("must parse");
        assert_eq!(host_port.to_string(), input);
    }

    #[rstest]
    // We use None here, because ParseIntErrors cannot be constructed outside of std. As such, it is
    // impossoble to fully qualify the error we expect in cases where port parsing fails.
    #[case("localhost:65536", None)]
    #[case("localhost:", None)]
    #[case("with space:", Some(ParseHostPortError::InvalidHost { source: ParseError::IdnaError }))]
    #[case("with space", Some(ParseHostPortError::InvalidHost { source: ParseError::IdnaError }))]
    #[case(":", Some(ParseHostPortError::InvalidHost { source: ParseError::EmptyHost }))]
    #[case("", Some(ParseHostPortError::EmptyInput))]
    fn invalid_host_port(#[case] input: &str, #[case] expected_error: Option<ParseHostPortError>) {
        let error = HostPort::from_str(input).expect_err("must not parse");
        if let Some(expected_error) = expected_error {
            assert_eq!(error, expected_error)
        }
    }
}
