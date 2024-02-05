use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Object used to create a space.
pub struct SpaceCreate {
    /// Space ID, must be unique. Note that a few spaces are reserved:
    ///
    /// `space`, `cdf`, `dms`, `pg3`, `shared`, `system`, `node`, `edge`.
    pub space: String,
    /// Space description.
    pub description: Option<String>,
    /// Human readable space name.
    pub name: Option<String>,
    /// Whether this space is a global space.
    pub intentionally_breaking_change: bool,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Description of a data modeling space.
pub struct Space {
    /// Space ID, must be unique.
    pub space: String,
    /// Space description.
    pub description: Option<String>,
    /// Human readable space name.
    pub name: Option<String>,
    /// Time this space was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this space was last modified, in milliseconds since epoch.
    pub last_updated_time: i64,
    /// Whether this space is a global space.
    pub is_global: bool,
}
