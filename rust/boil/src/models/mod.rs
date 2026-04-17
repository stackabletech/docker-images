use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TagList {
    // #[serde(rename = "name")]
    // pub _name: String,
    pub tags: Vec<String>,
}
