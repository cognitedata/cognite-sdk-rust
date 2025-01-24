use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{to_query, IntoParams, SetCursor};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A raw database
pub struct Database {
    /// Raw database name
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Request for deleting raw databases.
pub struct DeleteDatabasesRequest {
    /// List of databases to delete.
    pub items: Vec<Database>,
    /// Whether to allow deleting databases with tables in them.
    pub recursive: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A raw table.
pub struct Table {
    /// Raw table name.
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A raw row.
pub struct RawRow {
    /// Raw row key.
    pub key: String,
    /// Raw row columns
    pub columns: ::serde_json::Value,
    /// When this column was last updated, in milliseconds since epoch.
    pub last_updated_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// Create a raw row.
pub struct RawRowCreate {
    /// Raw row key.
    pub key: String,
    /// Raw row columns.
    pub columns: ::serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Delete a raw row.
pub struct DeleteRow {
    /// Raw row key.
    pub key: String,
}

#[derive(Debug, Default, Clone)]
/// Query for retrieving cursors for parallel read.
pub struct RetrieveCursorsQuery {
    /// Minimum last updated time, in milliseconds since epoch.
    pub min_last_updated_time: Option<i64>,
    /// Maximum last updated time, in milliseconds since epoch.
    pub max_last_updated_time: Option<i64>,
    /// Requested number of cursors.
    pub number_of_cursors: Option<i32>,
}

impl IntoParams for RetrieveCursorsQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query(
            "minLastUpdatedTime",
            &self.min_last_updated_time,
            &mut params,
        );
        to_query(
            "maxLastUpdatedTime",
            &self.max_last_updated_time,
            &mut params,
        );
        to_query("numberOfCursors", &self.number_of_cursors, &mut params);
        params
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Clone)]
/// Query for retrieving raw rows from a table.
pub struct RetrieveRowsQuery {
    /// Maximum number of rows to return.
    pub limit: Option<i32>,
    /// List of columns to return. Can be left out to return all columns.
    pub columns: Option<Vec<String>>,
    /// Optional cursor for pagination.
    pub cursor: Option<String>,
    /// Minimum last updated time, in millisecond since epoch.
    pub min_last_updated_time: Option<i64>,
    /// Maximum last updated time, in milliseconds since epoch.
    pub max_last_updated_time: Option<i64>,
}

impl IntoParams for RetrieveRowsQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("limit", &self.limit, &mut params);
        if let Some(columns) = self.columns {
            to_query("columns", &Some(columns.join(",")), &mut params);
        }
        to_query("cursor", &self.cursor, &mut params);
        to_query(
            "minLastUpdatedTime",
            &self.min_last_updated_time,
            &mut params,
        );
        to_query(
            "maxLastUpdatedTime",
            &self.max_last_updated_time,
            &mut params,
        );
        params
    }
}

impl SetCursor for RetrieveRowsQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

#[derive(Debug, Default, Clone, Copy)]
/// Query for whether to create missing parents when working with
/// tables or rows.
pub struct EnsureParentQuery {
    /// Set to `true` to create missing parents.
    pub ensure_parent: Option<bool>,
}

impl EnsureParentQuery {
    /// Create a new ensure parent query.
    ///
    /// # Arguments
    ///
    /// * `ensure_parent` - `true` to create missing raw tables and destinations.
    pub fn new(ensure_parent: bool) -> Self {
        Self {
            ensure_parent: Some(ensure_parent),
        }
    }
}

impl IntoParams for EnsureParentQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("ensureParent", &self.ensure_parent, &mut params);
        params
    }
}

#[derive(Debug, Default, Clone)]
/// Query for retrieving all rows using partitioned reads from a table.
pub struct RetrieveAllPartitionedQuery {
    /// Minimum last updated time, in millisecond since epoch.
    pub min_last_updated_time: Option<i64>,
    /// Maximum last updated time, in milliseconds since epoch.
    pub max_last_updated_time: Option<i64>,
    /// Requested number of parallel reads.
    pub number_of_cursors: Option<i32>,
    /// List of columns to return. Can be left out to return all columns.
    pub columns: Option<Vec<String>>,
    /// Maximum number of rows to return per request.
    pub limit: Option<i32>,
}
