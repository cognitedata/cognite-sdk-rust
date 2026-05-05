use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{to_query, IntoParams, SetCursor};
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

#[derive(Default, Clone, Debug)]
/// Query for listing spaces.
pub struct SpaceQuery {
    /// Maximum number of spaces in the result.
    pub limit: Option<i32>,
    /// Optional cursor for pagination.
    pub cursor: Option<String>,
    /// Include global spaces.
    pub include_global: Option<bool>,
}

impl IntoParams for SpaceQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        to_query("limit", &self.limit, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("includeGlobal", &self.include_global, &mut params);
        params
    }
}

impl SetCursor for SpaceQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}
