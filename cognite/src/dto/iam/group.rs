use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{to_query, IntoParams};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A CDF group.
pub struct Group {
    /// Internal ID.
    pub id: u64,
    /// Human readable name.
    pub name: String,
    /// ID of the group in the source IdP.
    pub source_id: Option<String>,
    /// Group capabilities object.
    pub capabilities: ::serde_json::Value,
    /// Whether this group is deleted.
    pub is_deleted: bool,
    /// Time this group was deleted.
    pub deleted_time: Option<i64>,
    /// Custom, immutable application specific metadata. String key -> String value.
    /// Limits: Key are at most 32 bytes. Values are at most 512 bytes.
    /// Up to 16 key-value pairs. Total size is at most 4096.
    pub metadata: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Create a CDF group.
pub struct AddGroup {
    /// Human readable name.
    pub name: String,
    /// ID of the group in the source IdP.
    pub source_id: Option<String>,
    /// Group capabilities object.
    pub capabilities: ::serde_json::Value,
    /// Custom, immutable application specific metadata. String key -> String value.
    /// Limits: Key are at most 32 bytes. Values are at most 512 bytes.
    /// Up to 16 key-value pairs. Total size is at most 4096.
    pub metadata: Option<HashMap<String, String>>,
}

impl From<Group> for AddGroup {
    fn from(value: Group) -> Self {
        Self {
            name: value.name,
            source_id: value.source_id,
            capabilities: value.capabilities,
            metadata: value.metadata,
        }
    }
}

#[derive(Debug, Default)]
/// Query for groups.
pub struct GroupQuery {
    /// Include all groups, or just groups for the current user.
    pub all: Option<bool>,
}

impl IntoParams for GroupQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("all", &self.all, &mut params);
        params
    }
}
