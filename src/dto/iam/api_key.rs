use serde::{Deserialize, Serialize};

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
    pub created_time: u64,
    pub status: String,
    pub value: Option<String>,
}
