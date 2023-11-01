use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::ViewReference;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemId {
    pub space: String,
    pub external_id: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemIdOptionalVersion {
    pub space: String,
    pub external_id: String,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum TaggedContainerReference {
    Container(ItemId),
}

impl From<ItemId> for TaggedContainerReference {
    fn from(value: ItemId) -> Self {
        Self::Container(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UsedFor {
    Node,
    Edge,
    All,
}
