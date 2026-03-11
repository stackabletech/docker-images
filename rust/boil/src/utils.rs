use std::process::Command;

use semver::Version;

use crate::build::{cli::HostPort, platform::Architecture};

/// Formats and returns the image repository URI, eg. `oci.stackable.tech/sdp/opa`.
pub fn format_image_repository_uri(
    image_registry: &HostPort,
    registry_namespace: &str,
    image_name: &str,
) -> String {
    format!("{image_registry}/{registry_namespace}/{image_name}")
}

/// Formats and returns the image manifest URI, eg. `oci.stackable.tech/sdp/opa:1.4.2-stackable25.7.0-amd64`.
pub fn format_image_manifest_uri(image_repository_uri: &str, image_manifest_tag: &str) -> String {
    format!("{image_repository_uri}:{image_manifest_tag}")
}

/// Formats and returns the image index manifest tag, eg. `1.4.2-stackable25.7.0`.
pub fn format_image_index_manifest_tag(
    image_version: &str,
    vendor_tag_prefix: &str,
    vendor_image_version: &Version,
) -> String {
    format!("{image_version}-{vendor_tag_prefix}{vendor_image_version}")
}

/// Formats and returns the image manifest tag, eg. `1.4.2-stackable25.7.0-amd64`.
///
/// The `strip_architecture` parameter controls if the architecture is included in the tag.
pub fn format_image_manifest_tag(
    image_index_manifest_tag: &str,
    // TODO (@Techassi): Maybe turn this into an Option to get rid of the bool
    architecture: &Architecture,
    strip_architecture: bool,
) -> String {
    if strip_architecture {
        image_index_manifest_tag.to_owned()
    } else {
        format!("{image_index_manifest_tag}-{architecture}")
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
