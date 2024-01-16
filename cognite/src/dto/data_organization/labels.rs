use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::Identity;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Label, used for labeling CDF resources.
pub struct Label {
    /// Label external ID. Must be unique within the project.
    pub external_id: String,
    /// Human readable label name.
    pub name: String,
    /// Label description.
    pub description: Option<String>,
    /// Data set this label belongs to.
    pub data_set_id: Option<i64>,
    /// Time this file was created, in milliseconds since epoch.
    pub created_time: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Create a label.
pub struct AddLabel {
    /// Label external ID. Must be unique within the project.
    pub external_id: String,
    /// Human readable label name.
    pub name: String,
    /// Label description.
    pub description: Option<String>,
    /// Data set this label belongs to.
    pub data_set_id: Option<i64>,
}

impl From<Label> for AddLabel {
    fn from(label: Label) -> Self {
        AddLabel {
            external_id: label.external_id,
            name: label.name,
            description: label.description,
            data_set_id: label.data_set_id,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Filter used for listing labels.
pub struct LabelFilter {
    /// Include labels with this name.
    pub name: Option<String>,
    /// Include labels with this (case sensitive) prefix on external ID.
    pub external_id_prefix: Option<String>,
    /// Include labels with one of these data sets.
    pub data_set_ids: Option<Vec<Identity>>,
}
