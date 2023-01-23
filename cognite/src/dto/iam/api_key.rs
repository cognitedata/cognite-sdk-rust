use serde::{Deserialize, Serialize};

use crate::{to_query, AsParams};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyListResponse {
    pub items: Vec<ApiKey>,
    previous_cursor: Option<String>,
    next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    pub id: u64,
    pub service_account_id: u64,
    pub created_time: i64,
    pub status: String,
    pub value: Option<String>,
}

#[derive(Debug, Default)]
pub struct ApiKeyQuery {
    pub all: Option<bool>,
    pub service_account_id: Option<i64>,
    pub include_deleted: Option<bool>,
}

impl AsParams for ApiKeyQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("all", &self.all, &mut params);
        to_query("serviceAccountId", &self.service_account_id, &mut params);
        to_query("includeDeleted", &self.include_deleted, &mut params);
        params
    }
}
