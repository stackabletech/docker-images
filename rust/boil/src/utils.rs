use std::process::Command;

use semver::Version;
use url::Host;

use crate::build::platform::Architecture;

/// Formats and returns the image repository URI, eg. `oci.stackable.tech/sdp/opa`.
pub fn format_image_repository_uri(
    image_registry: &Host,
    registry_namespace: &str,
    image_name: &str,
) -> String {
    format!("{image_registry}/{registry_namespace}/{image_name}")
}

/// Formats and returns the image manifest URI, eg. `oci.stackable.tech/sdp/opa:1.4.2-stackable25.7.0-amd64`.
pub fn format_image_manifest_uri(
    image_repository_uri: &str,
    image_version: &str,
    sdp_image_version: &Version,
    architecture: &Architecture,
    strip_architecture: bool,
) -> String {
    if strip_architecture {
        format!("{image_repository_uri}:{image_version}-stackable{sdp_image_version}")
    } else {
        format!(
            "{image_repository_uri}:{image_version}-stackable{sdp_image_version}-{architecture}"
        )
    }
}

pub trait CommandExt {
    /// Adds an argument to the command if the `predicate` is `true`.
    fn arg_if<S>(&mut self, predicate: bool, arg: S) -> &mut Self
    where
        S: AsRef<std::ffi::OsStr>;
}

impl CommandExt for Command {
    fn arg_if<S>(&mut self, predicate: bool, arg: S) -> &mut Self
    where
        S: AsRef<std::ffi::OsStr>,
    {
        if predicate { self.arg(arg) } else { self }
    }
}
