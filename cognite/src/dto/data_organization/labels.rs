use serde::{Deserialize, Serialize};

use crate::Identity;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub external_id: String,
    pub name: String,
    pub description: Option<String>,
    pub data_set_id: Option<i64>,
    pub created_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddLabel {
    pub external_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<i64>,
}

impl From<&Label> for AddLabel {
    fn from(label: &Label) -> Self {
        AddLabel {
            external_id: label.external_id.clone(),
            name: label.name.clone(),
            description: label.description.clone(),
            data_set_id: label.data_set_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LabelFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_ids: Option<Vec<Identity>>,
}
