use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    Identity, IntoPatch, IntoPatchItem, Patch, Range, UpdateMap, UpdateSet, UpdateSetNull,
};

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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddDataSet {
    pub external_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub write_protected: bool,
}

impl From<DataSet> for AddDataSet {
    fn from(dataset: DataSet) -> Self {
        AddDataSet {
            external_id: dataset.external_id,
            name: dataset.name,
            description: dataset.description,
            metadata: dataset.metadata,
            write_protected: dataset.write_protected,
        }
    }
}

#[skip_serializing_none]
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PatchDataSet {
    pub external_id: Option<UpdateSetNull<String>>,
    pub name: Option<UpdateSetNull<String>>,
    pub description: Option<UpdateSetNull<String>>,
    pub metadata: Option<UpdateMap<String, String>>,
    pub write_protected: Option<UpdateSet<bool>>,
}

impl IntoPatch<Patch<PatchDataSet>> for DataSet {
    fn patch(self, ignore_nulls: bool) -> Patch<PatchDataSet> {
        Patch::<PatchDataSet> {
            id: to_idt!(self),
            update: PatchDataSet {
                external_id: None,
                name: self.name.patch(ignore_nulls),
                description: self.description.patch(ignore_nulls),
                metadata: self.metadata.patch(ignore_nulls),
                write_protected: self.write_protected.patch(ignore_nulls),
            },
        }
    }
}

impl IntoPatch<PatchDataSet> for AddDataSet {
    fn patch(self, ignore_nulls: bool) -> PatchDataSet {
        PatchDataSet {
            external_id: None,
            name: self.name.patch(ignore_nulls),
            description: self.description.patch(ignore_nulls),
            metadata: self.metadata.patch(ignore_nulls),
            write_protected: self.write_protected.patch(ignore_nulls),
        }
    }
}

impl From<DataSet> for Patch<PatchDataSet> {
    fn from(data_set: DataSet) -> Patch<PatchDataSet> {
        IntoPatch::<Patch<PatchDataSet>>::patch(data_set, false)
    }
}
