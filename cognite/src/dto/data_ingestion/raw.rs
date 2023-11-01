use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{to_query, AsParams, SetCursor};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDatabasesRequest {
    pub items: Vec<Database>,
    pub recursive: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawRow {
    pub key: String,
    pub columns: ::serde_json::Value,
    pub last_updated_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RawRowCreate {
    pub key: String,
    pub columns: ::serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteRow {
    pub key: String,
}

#[derive(Debug, Default, Clone)]
pub struct RetrieveCursorsQuery {
    pub min_last_updated_time: Option<i64>,
    pub max_last_updated_time: Option<i64>,
    pub number_of_cursors: Option<i32>,
}

impl AsParams for RetrieveCursorsQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
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
pub struct RetrieveRowsQuery {
    pub limit: Option<i32>,
    pub columns: Option<Vec<String>>,
    pub cursor: Option<String>,
    pub min_last_updated_time: Option<i64>,
    pub max_last_updated_time: Option<i64>,
}

impl AsParams for RetrieveRowsQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
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

pub struct EnsureParentQuery {
    pub ensure_parent: Option<bool>,
}

impl AsParams for EnsureParentQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("ensureParent", &self.ensure_parent, &mut params);
        params
    }
}
