use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameMetadataResponse {
    pub entity: Entity,
    pub links: Vec<Link>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: Option<String>,
    pub relationship: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub operations: Vec<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub method: String,
}
