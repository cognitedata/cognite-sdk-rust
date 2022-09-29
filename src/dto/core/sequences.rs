use crate::{EqIdentity, Identity, Patch, Range, UpdateList, UpdateMap, UpdateSet, UpdateSetNull};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum SequenceValueType {
    #[serde(rename = "DOUBLE")]
    Double,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "LONG")]
    Long,
}

impl Default for SequenceValueType {
    fn default() -> Self {
        SequenceValueType::Double
    }
}

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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddSequence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    pub columns: Vec<SequenceColumn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<i64>,
}

impl From<&Sequence> for AddSequence {
    fn from(sequence: &Sequence) -> Self {
        AddSequence {
            external_id: sequence.external_id.clone(),
            name: sequence.name.clone(),
            description: sequence.description.clone(),
            asset_id: sequence.asset_id.clone(),
            metadata: sequence.metadata.clone(),
            columns: sequence.columns.clone(),
            data_set_id: sequence.data_set_id.clone(),
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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSequenceColumns {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify: Option<Vec<Patch<PatchSequenceColumn>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<SequenceColumn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
}

impl From<UpdateList<SequenceColumn, String>> for UpdateSequenceColumns {
    fn from(upd: UpdateList<SequenceColumn, String>) -> Self {
        Self {
            modify: None,
            add: upd.add,
            remove: upd.remove,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchSequenceColumn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<UpdateSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<UpdateMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchSequence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<UpdateMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<UpdateSequenceColumns>,
}

impl From<&Sequence> for Patch<PatchSequence> {
    fn from(sequence: &Sequence) -> Self {
        Self {
            id: Identity::Id { id: sequence.id },
            update: PatchSequence {
                name: Some(sequence.name.clone().into()),
                description: Some(sequence.description.clone().into()),
                asset_id: Some(sequence.asset_id.clone().into()),
                external_id: Some(sequence.external_id.clone().into()),
                metadata: Some(sequence.metadata.clone().into()),
                data_set_id: Some(sequence.data_set_id.clone().into()),
                columns: None,
            },
        }
    }
}

impl From<&AddSequence> for PatchSequence {
    fn from(sequence: &AddSequence) -> Self {
        PatchSequence {
            name: Some(sequence.name.clone().into()),
            description: Some(sequence.description.clone().into()),
            asset_id: Some(sequence.asset_id.clone().into()),
            external_id: Some(sequence.external_id.clone().into()),
            metadata: Some(sequence.metadata.clone().into()),
            data_set_id: Some(sequence.data_set_id.clone().into()),
            columns: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SequenceFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_asset_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_subtree_ids: Option<Vec<Identity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_ids: Option<Vec<Identity>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SequenceSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicSequenceColumn {
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub value_type: SequenceValueType,
}

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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveSequenceRows {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    #[serde(flatten)]
    pub id: Identity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveLatestSequenceRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
