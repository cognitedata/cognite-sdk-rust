use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroupListResponse {
    pub items: Vec<Group>,
    previous_cursor: Option<String>,
    next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub source_id: Option<u64>,
    pub capabilities: ::serde_json::Value,
    pub is_deleted: Option<bool>,
    pub deleted_time: Option<i64>,
}
