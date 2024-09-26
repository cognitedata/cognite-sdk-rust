use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    models::views::{ViewCreateOrReference, ViewDefinitionOrReference},
    to_query, IntoParams, SetCursor,
};

mod extensions;
pub use extensions::{
    files::{CogniteExtractorFile, FileObject},
    FromReadable, IntoWritable,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Create a data model.
pub struct DataModelCreate {
    /// Data model space.
    pub space: String,
    /// Data model external ID.
    pub external_id: String,
    /// Data model name.
    pub name: Option<String>,
    /// Data model description.
    pub description: Option<String>,
    /// Data model version.
    pub version: String,
    /// Views in data model.
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
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// A CDF data model.
pub struct DataModel {
    /// Data model space.
    pub space: String,
    /// Data model external ID.
    pub external_id: String,
    /// Data model name.
    pub name: Option<String>,
    /// Data model description.
    pub description: Option<String>,
    /// Data model version.
    pub version: String,
    /// Views in data model.
    pub views: Option<Vec<ViewDefinitionOrReference>>,
    /// Time this data model was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this data model was last updated, in milliseconds since epoch.
    pub last_updated_time: i64,
    /// Whether this data model is global (defined by CDF) or project specific.
    pub is_global: bool,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// ID of a data model
pub struct DataModelId {
    /// Data model space.
    pub space: String,
    /// Data model external ID.
    pub external_id: String,
    /// Data model version. This is required for some endpoints, but not all.
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default)]
/// Query for listing data models.
pub struct DataModelQuery {
    /// Optional cursor for pagination.
    pub cursor: Option<String>,
    /// Maximum number of data models to retrieve. Default is 10, maximum is 1000.
    pub limit: Option<i32>,
    /// Whether to expand the referenced views inline in the returned result.
    pub inline_views: Option<bool>,
    /// Filter by data model space.
    pub space: Option<String>,
    /// Whether to include all versions, or just the latest version.
    pub all_versions: Option<bool>,
    /// Whether to include global data models.
    pub include_global: Option<bool>,
}

impl SetCursor for DataModelQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl IntoParams for DataModelQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        to_query("cursor", &self.cursor, &mut params);
        to_query("limit", &self.limit, &mut params);
        to_query("inlineViews", &self.inline_views, &mut params);
        to_query("space", &self.space, &mut params);
        to_query("allVersions", &self.all_versions, &mut params);
        to_query("includeGlobal", &self.include_global, &mut params);
        params
    }
}
