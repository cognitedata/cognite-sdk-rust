use serde::{Deserialize, Serialize};

use crate::{to_query, AsParams};

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
