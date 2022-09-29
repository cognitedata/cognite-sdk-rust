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
    pub description: Option<String>,
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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LabelFilter {
    pub name: Option<String>,
    pub external_id_prefix: Option<String>,
    pub data_set_ids: Option<Vec<Identity>>,
}
