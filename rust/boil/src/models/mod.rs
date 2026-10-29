use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TagList {
    // #[serde(rename = "name")]
    // pub _name: String,
    pub tags: Vec<String>,
}

// TODO (@Techassi): We should eventually use the complete, upstream types from oci-spec
/// A partial OCI manifest.
#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub layers: Vec<ManifestLayer>,
}

/// A partial OCI manifest layer.
#[derive(Debug, Deserialize)]
pub struct ManifestLayer {
    pub size: u64,
}
