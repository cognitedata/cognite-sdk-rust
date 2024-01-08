use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Identity, IntoPatch, IntoPatchItem, Patch, Range, UpdateMap, UpdateSetNull};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// A data set grouping data in CDF.
pub struct DataSet {
    /// Data set internal ID.
    pub id: i64,
    /// Time this data set was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this data set was last modified, in milliseconds since epoch.
    pub last_updated_time: i64,
    /// Data set external ID. Must be unique within the project.
    pub external_id: Option<String>,
    /// Human readable data set name.
    pub name: Option<String>,
    /// Data set description.
    pub description: Option<String>,
    /// Custom, application specific metadata. String key -> String value.
    /// Limits: Maximum length of key is 128 bytes, value 10240 bytes,
    /// up to 256 key-value pairs, of total size at most 10240.
    pub metadata: Option<HashMap<String, String>>,
    /// To write data to a write-protected data set, you need to be a member of
    /// a group that has the "datasets:owner" action for the data set.
    pub write_protected: bool,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Create a CDF data set.
pub struct AddDataSet {
    /// Data set external ID. Must be unique within the project.
    pub external_id: Option<String>,
    /// Human readable data set name.
    pub name: Option<String>,
    /// Data set description.
    pub description: Option<String>,
    /// Custom, application specific metadata. String key -> String value.
    /// Limits: Maximum length of key is 128 bytes, value 10240 bytes,
    /// up to 256 key-value pairs, of total size at most 10240.
    pub metadata: Option<HashMap<String, String>>,
    /// To write data to a write-protected data set, you need to be a member of
    /// a group that has the "datasets:owner" action for the data set.
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
/// Filter for listing data sets.
pub struct DataSetFilter {
    /// Filter based on metadata.
    pub metadata: Option<HashMap<String, String>>,
    /// Range of timestamps for `created_time`.
    pub created_time: Option<Range<i64>>,
    /// Range of timestamps for `last_updated_time`.
    pub last_updated_time: Option<Range<i64>>,
    /// Filter by this (case-sensitive) prefix for the external ID.
    pub external_id_prefix: Option<String>,
    /// Filter on data set `writeProtected`.
    pub write_protected: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Result for aggregating data sets.
pub struct DataSetsCount {
    /// Data set count.
    pub count: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Update data sets.
pub struct PatchDataSet {
    /// Data set external ID. Must be unique within the project.
    pub external_id: Option<UpdateSetNull<String>>,
    /// Data set name.
    pub name: Option<UpdateSetNull<String>>,
    /// Data set description.
    pub description: Option<UpdateSetNull<String>>,
    /// Custom, application specific metadata. String key -> String value.
    /// Limits: Maximum length of key is 128 bytes, value 10240 bytes,
    /// up to 256 key-value pairs, of total size at most 10240.
    pub metadata: Option<UpdateMap<String, String>>,
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
        }
    }
}

impl From<DataSet> for Patch<PatchDataSet> {
    fn from(data_set: DataSet) -> Patch<PatchDataSet> {
        IntoPatch::<Patch<PatchDataSet>>::patch(data_set, false)
    }
}
