use std::path::PathBuf;

use clap::{Args, ValueHint, value_parser};
use semver::Version;
use url::Host;

use crate::{
    build::{
        docker::BuildArgument,
        image::Image,
        platform::{Architecture, TargetPlatform},
    },
    cli::parse_image_version,
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
    #[arg(
        short, long,
        default_value_t = Self::default_registry(),
        value_parser = Host::parse,
        value_hint = ValueHint::Hostname,
        help_heading = "Registry Options"
    )]
    pub registry: Host,

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

    /// Override the target containerfile used, points to <PRODUCT>/<TARGET_CONTAINERFILE>.
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
        help_heading = "Build Options"
    )]
    pub docker_build_arguments: Vec<BuildArgument>,

    /// Write target image tags to <EXPORT_FILE>. Useful for signing or other follow-up CI steps.
    #[arg(
        long,
        alias = "export-tags-file",
        help_heading = "Build Options",
        value_name = "EXPORT_FILE",
        value_hint = ValueHint::FilePath,
        value_parser = value_parser!(PathBuf),
        default_missing_value = "boil-target-tags",
        num_args(0..=1)
    )]
    pub export_image_manifest_uris: Option<PathBuf>,

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

    fn default_architecture() -> TargetPlatform {
        TargetPlatform::Linux(Architecture::Amd64)
    }

    fn default_registry() -> Host {
        Host::Domain(String::from("oci.stackable.tech"))
    }

    fn default_target_containerfile() -> PathBuf {
        PathBuf::from("Dockerfile")
    }
}
