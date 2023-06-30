use serde::{Deserialize, Serialize};

use crate::models::ViewReference;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemId {
    pub space: String,
    pub external_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemIdOptionalVersion {
    pub space: String,
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SourceReference {
    View(ViewReference),
    Container(ItemId),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpaceId {
    pub space: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum TaggedViewReference {
    View(ViewReference),
}

impl From<ViewReference> for TaggedViewReference {
    fn from(value: ViewReference) -> Self {
        Self::View(value)
    }
}
