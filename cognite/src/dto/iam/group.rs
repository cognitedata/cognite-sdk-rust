use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{to_query, AsParams};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroupListResponse {
    pub items: Vec<Group>,
    previous_cursor: Option<String>,
    next_cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub source_id: Option<String>,
    pub capabilities: ::serde_json::Value,
    pub is_deleted: Option<bool>,
    pub deleted_time: Option<i64>,
    pub metadata: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddGroup {
    pub name: String,
    pub source_id: Option<String>,
    pub capabilities: ::serde_json::Value,
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
pub struct GroupQuery {
    pub all: Option<bool>,
}

impl AsParams for GroupQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("all", &self.all, &mut params);
        params
    }
}
