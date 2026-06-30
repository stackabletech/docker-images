use std::{process::Command, str::FromStr};

use snafu::{ResultExt, Snafu};

use crate::{cli::HostPort, core::platform::Architecture};

// FIXME (@Techassi): We should pull this in from a central pice of code, like stackable-shared.
// stackable-shared needs to add a few features to be able to properly select _only_ what is needed
// without pulling in too many unused deps.
pub trait VersionExt {
    fn is_floating(&self) -> bool;

    fn floating(&self) -> String;
}

impl VersionExt for semver::Version {
    fn is_floating(&self) -> bool {
        self.major == 0
            && self.minor == 0
            && self.patch == 0
            && (self.pre.starts_with("pr") || self.pre.as_str() == "dev")
    }

    fn floating(&self) -> String {
        if self.is_floating() {
            self.to_string()
        } else {
            format!("{major}.{minor}", major = self.major, minor = self.minor)
        }
    }
}

#[derive(Debug, Snafu)]
pub enum ParseFloatingVendorVersionError {
    #[snafu(display("failed to parse {version:?} as semantic version"))]
    ParseSemanticVersion {
        source: semver::Error,
        version: String,
    },
}

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
    vendor_image_version: &str,
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

/// Formats and returns the registry-specific env var name, eg. `BOIL_REGISTRY_TOKEN_OCI_STACKABLE_TECH`.
pub fn format_registry_token_env_var_name(registry_uri: &str) -> String {
    format!(
        "BOIL_REGISTRY_TOKEN_{registry_uri}",
        registry_uri = registry_uri.replace(['.', '-'], "_").to_uppercase()
    )
}

pub fn parse_floating_vendor_version(
    vendor_image_version: &str,
    floating_tag: bool,
) -> Result<Option<String>, ParseFloatingVendorVersionError> {
    if floating_tag {
        let version =
            semver::Version::from_str(vendor_image_version).context(ParseSemanticVersionSnafu {
                version: vendor_image_version,
            })?;

        // Return None because the selected version is already considered a floating version as is.
        // There is no need to add a second, identical tag to the image.
        if version.is_floating() {
            return Ok(None);
        }

        Ok(Some(version.floating()))
    } else {
        Ok(None)
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
