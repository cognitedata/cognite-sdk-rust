use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpaceCreate {
    pub space: String,
    pub description: Option<String>,
    pub name: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    pub space: String,
    pub description: Option<String>,
    pub name: Option<String>,
    pub created_time: i64,
    pub last_updated_time: i64,
}
