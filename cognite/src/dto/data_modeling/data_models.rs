use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    models::{ViewCreateOrReference, ViewDefinitionOrReference},
    to_query, AsParams, SetCursor,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataModelCreate {
    pub space: String,
    pub external_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: String,
    pub views: Option<Vec<ViewCreateOrReference>>,
}

impl From<DataModel> for DataModelCreate {
    fn from(value: DataModel) -> Self {
        DataModelCreate {
            space: value.space,
            external_id: value.external_id,
            name: value.name,
            description: value.description,
            version: value.version,
            views: value
                .views
                .map(|views| views.into_iter().map(|v| v.into()).collect()),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataModel {
    pub space: String,
    pub external_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: String,
    pub views: Option<Vec<ViewDefinitionOrReference>>,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub is_global: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataModelId {
    pub space: String,
    pub external_id: String,
    pub version: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataModelFilter {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub spaces: Option<Vec<String>>,
    pub all_versions: Option<bool>,
    pub include_global: Option<bool>,
}

impl SetCursor for DataModelFilter {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

#[derive(Clone, Debug)]
pub struct DataModelQuery {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub inline_views: Option<bool>,
    pub space: Option<String>,
    pub all_versions: Option<bool>,
    pub include_global: Option<bool>,
}

impl SetCursor for DataModelQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl AsParams for DataModelQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        to_query("cursor", &self.cursor, &mut params);
        to_query("limit", &self.limit, &mut params);
        to_query("inline_views", &self.inline_views, &mut params);
        to_query("space", &self.space, &mut params);
        to_query("all_versions", &self.all_versions, &mut params);
        to_query("include_global", &self.include_global, &mut params);
        params
    }
}
