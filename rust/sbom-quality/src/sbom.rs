//! The parts of CycloneDX and SPDX documents that the checks look at.
//!
//! Every field is optional, so that a missing field produces a finding of the check that needs
//! it instead of failing the whole document. Only a field of the wrong type makes a document
//! unreadable.

use serde::Deserialize;
use serde_json::Value;

/// Removes every `null` from objects, recursively.
///
/// A field that is `null` means the same as a missing one, but serde only applies the default of
/// a field that is missing. mergebom writes `"documentDescribes": null`, for example.
pub fn remove_nulls(value: &mut Value) {
    match value {
        Value::Object(object) => {
            object.retain(|_, value| !value.is_null());
            object.values_mut().for_each(remove_nulls);
        }
        Value::Array(items) => items.iter_mut().for_each(remove_nulls),
        _ => {}
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CycloneDx {
    pub bom_format: Option<String>,
    pub spec_version: Option<String>,
    pub serial_number: Option<String>,
    pub metadata: Metadata,
    pub components: Vec<Component>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Metadata {
    pub timestamp: Option<String>,
    pub authors: Vec<Contact>,
    pub lifecycles: Vec<Lifecycle>,

    /// Either an object with `components` or the deprecated list form, so it is kept as JSON.
    pub tools: Value,

    pub component: Option<Component>,
    pub properties: Vec<Property>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: Option<String>,

    #[serde(rename = "bom-ref")]
    pub bom_ref: Option<String>,

    pub name: Option<String>,
    pub version: Option<String>,
    pub purl: Option<String>,
    pub description: Option<String>,

    /// Only checked for presence, so the license choices are kept as JSON.
    pub licenses: Vec<Value>,

    pub supplier: Option<Entity>,
    pub manufacturer: Option<Entity>,
    pub properties: Vec<Property>,
}

impl Component {
    pub fn property(&self, name: &str) -> Option<&str> {
        self.properties
            .iter()
            .find(|property| property.name.as_deref() == Some(name))
            .and_then(|property| property.value.as_deref())
    }

    /// The syft cataloger that found the component.
    pub fn found_by(&self) -> Option<&str> {
        self.property("syft:package:foundBy")
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Entity {
    pub name: Option<String>,
    pub url: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Contact {
    pub name: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Lifecycle {
    pub phase: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Property {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Dependency {
    #[serde(rename = "ref")]
    pub reference: Option<String>,

    pub depends_on: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Spdx {
    pub spdx_version: Option<String>,
    pub data_license: Option<String>,

    #[serde(rename = "SPDXID")]
    pub spdx_id: Option<String>,

    pub document_namespace: Option<String>,
    pub creation_info: CreationInfo,
    pub document_describes: Vec<String>,
    pub packages: Vec<Package>,
    pub relationships: Vec<Relationship>,
    pub has_extracted_licensing_infos: Vec<ExtractedLicense>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CreationInfo {
    pub created: Option<String>,
    pub creators: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Package {
    #[serde(rename = "SPDXID")]
    pub spdx_id: Option<String>,

    pub name: Option<String>,
    pub version_info: Option<String>,
    pub supplier: Option<String>,
    pub download_location: Option<String>,
    pub license_declared: Option<String>,
    pub license_concluded: Option<String>,
    pub primary_package_purpose: Option<String>,
    pub external_refs: Vec<ExternalReference>,
}

impl Package {
    pub fn purls(&self) -> Vec<&str> {
        self.external_refs
            .iter()
            .filter(|reference| reference.reference_type.as_deref() == Some("purl"))
            .filter_map(|reference| reference.reference_locator.as_deref())
            .collect()
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ExternalReference {
    pub reference_type: Option<String>,
    pub reference_locator: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Relationship {
    pub spdx_element_id: Option<String>,
    pub relationship_type: Option<String>,
    pub related_spdx_element: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ExtractedLicense {
    pub license_id: Option<String>,
}
