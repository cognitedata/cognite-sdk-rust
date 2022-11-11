use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Identity, Patch, Range, UpdateMap, UpdateSet, UpdateSetNull};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataSet {
    pub id: i64,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub external_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub write_protected: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddDataSet {
    pub external_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub write_protected: bool,
}

impl From<&DataSet> for AddDataSet {
    fn from(dataset: &DataSet) -> Self {
        AddDataSet {
            external_id: dataset.external_id.clone(),
            name: dataset.name.clone(),
            description: dataset.description.clone(),
            metadata: dataset.metadata.clone(),
            write_protected: dataset.write_protected,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataSetFilter {
    pub metadata: Option<HashMap<String, String>>,
    pub created_time: Option<Range<i64>>,
    pub last_updated_time: Option<Range<i64>>,
    pub external_id_prefix: Option<String>,
    pub write_protected: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataSetsCount {
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchDataSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<UpdateMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_protected: Option<UpdateSet<bool>>,
}

impl From<&DataSet> for Patch<PatchDataSet> {
    fn from(data_set: &DataSet) -> Patch<PatchDataSet> {
        Patch::<PatchDataSet> {
            id: to_idt!(data_set),
            update: PatchDataSet {
                external_id: Some(data_set.external_id.clone().into()),
                name: Some(data_set.name.clone().into()),
                description: Some(data_set.description.clone().into()),
                metadata: Some(data_set.metadata.clone().into()),
                write_protected: Some(data_set.write_protected.into()),
            },
        }
    }
}

impl From<&AddDataSet> for PatchDataSet {
    fn from(data_set: &AddDataSet) -> Self {
        PatchDataSet {
            external_id: Some(data_set.external_id.clone().into()),
            name: Some(data_set.name.clone().into()),
            description: Some(data_set.description.clone().into()),
            metadata: Some(data_set.metadata.clone().into()),
            write_protected: Some(data_set.write_protected.into()),
        }
    }
}
