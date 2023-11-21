use crate::{
    models::FdmFilter, EqIdentity, Identity, IntoPatch, IntoPatchItem, Partition, Patch, Range,
    SetCursor, UpdateList, UpdateMap, UpdateSet, UpdateSetNull, WithPartition,
};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use super::CoreSortItem;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "camelCase")]
pub enum SequenceValueType {
    #[serde(rename = "DOUBLE")]
    #[default]
    Double,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "LONG")]
    Long,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SequenceColumn {
    pub name: Option<String>,
    pub external_id: String,
    pub description: Option<String>,
    pub value_type: SequenceValueType,
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing)]
    pub created_time: Option<i64>,
    #[serde(skip_serializing)]
    pub last_updated_time: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    pub id: i64,
    pub external_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub asset_id: Option<i64>,
    pub metadata: Option<HashMap<String, String>>,
    pub columns: Vec<SequenceColumn>,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub data_set_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddSequence {
    pub external_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub asset_id: Option<i64>,
    pub metadata: Option<HashMap<String, String>>,
    pub columns: Vec<SequenceColumn>,
    pub data_set_id: Option<i64>,
}

impl From<Sequence> for AddSequence {
    fn from(sequence: Sequence) -> Self {
        AddSequence {
            external_id: sequence.external_id,
            name: sequence.name,
            description: sequence.description,
            asset_id: sequence.asset_id,
            metadata: sequence.metadata,
            columns: sequence.columns,
            data_set_id: sequence.data_set_id,
        }
    }
}

impl EqIdentity for AddSequence {
    fn eq(&self, id: &Identity) -> bool {
        match id {
            Identity::Id { id: _ } => false,
            Identity::ExternalId { external_id } => self.external_id.as_ref() == Some(external_id),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSequenceColumns {
    pub modify: Option<Vec<Patch<PatchSequenceColumn>>>,
    pub add: Option<Vec<SequenceColumn>>,
    pub remove: Option<Vec<String>>,
}

impl From<UpdateList<SequenceColumn, String>> for UpdateSequenceColumns {
    fn from(upd: UpdateList<SequenceColumn, String>) -> Self {
        match upd {
            UpdateList::AddRemove { add, remove } => Self {
                modify: None,
                add,
                remove,
            },
            UpdateList::Set { set } => Self {
                add: Some(set),
                modify: None,
                remove: None,
            },
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PatchSequenceColumn {
    pub description: Option<UpdateSetNull<String>>,
    pub external_id: Option<UpdateSet<String>>,
    pub name: Option<UpdateSetNull<String>>,
    pub metadata: Option<UpdateMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PatchSequence {
    pub name: Option<UpdateSetNull<String>>,
    pub description: Option<UpdateSetNull<String>>,
    pub asset_id: Option<UpdateSetNull<i64>>,
    pub external_id: Option<UpdateSetNull<String>>,
    pub metadata: Option<UpdateMap<String, String>>,
    pub data_set_id: Option<UpdateSetNull<i64>>,
    pub columns: Option<UpdateSequenceColumns>,
}

impl IntoPatch<Patch<PatchSequence>> for Sequence {
    fn patch(self, ignore_nulls: bool) -> Patch<PatchSequence> {
        Patch::<PatchSequence> {
            id: to_idt!(self),
            update: PatchSequence {
                name: self.name.patch(ignore_nulls),
                description: self.description.patch(ignore_nulls),
                asset_id: self.asset_id.patch(ignore_nulls),
                external_id: self.external_id.patch(ignore_nulls),
                metadata: self.metadata.patch(ignore_nulls),
                data_set_id: self.data_set_id.patch(ignore_nulls),
                columns: None,
            },
        }
    }
}

impl IntoPatch<PatchSequence> for AddSequence {
    fn patch(self, ignore_nulls: bool) -> PatchSequence {
        PatchSequence {
            name: self.name.patch(ignore_nulls),
            description: self.description.patch(ignore_nulls),
            asset_id: self.asset_id.patch(ignore_nulls),
            external_id: self.external_id.patch(ignore_nulls),
            metadata: self.metadata.patch(ignore_nulls),
            data_set_id: self.data_set_id.patch(ignore_nulls),
            columns: None,
        }
    }
}

impl From<Sequence> for Patch<PatchSequence> {
    fn from(sequence: Sequence) -> Self {
        IntoPatch::<Patch<PatchSequence>>::patch(sequence, false)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SequenceFilter {
    pub name: Option<String>,
    pub external_id_prefix: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub asset_ids: Option<Vec<i64>>,
    pub root_asset_ids: Option<Vec<i64>>,
    pub asset_subtree_ids: Option<Vec<Identity>>,
    pub created_time: Option<Range<i64>>,
    pub last_updated_time: Option<Range<i64>>,
    pub data_set_ids: Option<Vec<Identity>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SequenceFilterRequest {
    pub filter: Option<SequenceFilter>,
    pub advanced_filter: Option<FdmFilter>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
    pub partition: Option<Partition>,
    pub sort: Option<Vec<CoreSortItem>>,
}

impl SetCursor for SequenceFilterRequest {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl WithPartition for SequenceFilterRequest {
    fn with_partition(&self, partition: Partition) -> Self {
        let mut copy = self.clone();
        copy.partition = Some(partition);
        copy
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SequenceSearch {
    pub name: Option<String>,
    pub description: Option<String>,
    pub query: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum SequenceRowValue {
    String(String),
    Long(i64),
    Double(f64),
    Null(()),
}

impl Default for SequenceRowValue {
    fn default() -> Self {
        SequenceRowValue::Null(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SequenceRow {
    pub row_number: i64,
    pub values: Vec<SequenceRowValue>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicSequenceColumn {
    pub external_id: String,
    pub name: Option<String>,
    pub value_type: SequenceValueType,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveSequenceRowsResponse {
    pub id: i64,
    pub external_id: Option<String>,
    pub columns: Vec<BasicSequenceColumn>,
    pub rows: Vec<SequenceRow>,
    pub next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsertSequenceRows {
    pub columns: Vec<String>,
    pub rows: Vec<SequenceRow>,
    #[serde(flatten)]
    pub id: Identity,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveSequenceRows {
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
    pub columns: Option<Vec<String>>,
    #[serde(flatten)]
    pub id: Identity,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveLastSequenceRow {
    pub columns: Option<Vec<String>>,
    pub before: Option<i64>,
    #[serde(flatten)]
    pub id: Identity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSequenceRows {
    pub rows: Vec<i64>,
    #[serde(flatten)]
    pub id: Identity,
}
