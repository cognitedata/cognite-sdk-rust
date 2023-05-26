use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{ViewCreateDefinition, ViewDefinition, ViewReference};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct DataModelCreate {
    pub space: String,
    pub external_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: String,
    pub views: Option<Vec<DataModelCreateProperty>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct DataModel {
    pub space: String,
    pub external_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: String,
    pub views: Option<Vec<DataModelProperty>>,
    pub created_time: i64,
    pub last_updated_time: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum DataModelCreateProperty {
    ViewCreateDefinition(ViewCreateDefinition),
    ViewReference(ViewReference),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum DataModelProperty {
    ViewDefinition(ViewDefinition),
    ViewReference(ViewReference),
}
