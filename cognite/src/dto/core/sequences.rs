use crate::{
    AdvancedFilter, EqIdentity, Identity, IntoPatch, IntoPatchItem, Partition, Patch, Range,
    SetCursor, UpdateList, UpdateMap, UpdateSet, UpdateSetNull, UpsertOptions, WithPartition,
};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use super::common::CoreSortItem;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "camelCase")]
/// Type of sequence value.
pub enum SequenceValueType {
    #[serde(rename = "DOUBLE")]
    #[default]
    /// Double precision floating point.
    Double,
    #[serde(rename = "STRING")]
    /// String.
    String,
    #[serde(rename = "LONG")]
    /// 64-bit integer.
    Long,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// Description of a single sequence column
pub struct SequenceColumn {
    /// Name of the sequence column.
    pub name: Option<String>,
    /// External ID of the sequence column. Must be unique for a given sequence.
    pub external_id: String,
    /// Description of the sequence column.
    pub description: Option<String>,
    /// Type of column value.
    pub value_type: SequenceValueType,
    /// Custom, application specific metadata. String key -> String value.
    /// Maximum length of key is 128 bytes, up to 256 key-value pairs,
    /// up to a total size of 10000 bytes across all keys and values.
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing)]
    /// Time this sequence column was created, in milliseconds since epoch.
    pub created_time: Option<i64>,
    #[serde(skip_serializing)]
    /// Time this sequence column was last updated, in milliseconds since epoch.
    pub last_updated_time: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// A CDF sequence.
pub struct Sequence {
    /// Sequence internal ID.
    pub id: i64,
    /// External ID of the sequence. Must be unique within the project.
    pub external_id: Option<String>,
    /// Name of the sequence.
    pub name: Option<String>,
    /// Description of the sequence.
    pub description: Option<String>,
    /// ID of asset this sequence belongs to.
    pub asset_id: Option<i64>,
    /// Custom, application specific metadata. String key -> String value.
    /// Maximum length of key is 128 bytes, up to 256 key-value pairs,
    /// up to a total size of 10000 bytes across all keys and values.
    pub metadata: Option<HashMap<String, String>>,
    /// List of columns in this sequence.
    pub columns: Vec<SequenceColumn>,
    /// Time this sequence was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this sequence was last updated, in milliseconds since epoch.
    pub last_updated_time: i64,
    /// ID of the data set this sequence belongs to.
    pub data_set_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Create a new sequence.
pub struct AddSequence {
    /// External ID of the sequence. Must be unique within the project.
    pub external_id: Option<String>,
    /// Name of the sequence.
    pub name: Option<String>,
    /// Description of the sequence.
    pub description: Option<String>,
    /// ID of asset this sequence belongs to.
    pub asset_id: Option<i64>,
    /// Custom, application specific metadata. String key -> String value.
    /// Maximum length of key is 128 bytes, up to 256 key-value pairs,
    /// up to a total size of 10000 bytes across all keys and values.
    pub metadata: Option<HashMap<String, String>>,
    /// List of columns in this sequence.
    pub columns: Vec<SequenceColumn>,
    /// ID of the data set this sequence belongs to.
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
/// Update the columns of a sequence.
pub struct UpdateSequenceColumns {
    /// List of column updates.
    pub modify: Option<Vec<Patch<PatchSequenceColumn>>>,
    /// List of column definitions to add.
    pub add: Option<Vec<SequenceColumn>>,
    /// List of columns to remove.
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
/// Update a sequence column.
pub struct PatchSequenceColumn {
    /// Description of the sequence.
    pub description: Option<UpdateSetNull<String>>,
    /// External ID of the sequence column. Must be unique for a given sequence.
    pub external_id: Option<UpdateSet<String>>,
    /// Name of the sequence column.
    pub name: Option<UpdateSetNull<String>>,
    /// Custom, application specific metadata. String key -> String value.
    /// Maximum length of key is 128 bytes, up to 256 key-value pairs,
    /// up to a total size of 10000 bytes across all keys and values.
    pub metadata: Option<UpdateMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Update a sequence.
pub struct PatchSequence {
    /// Name of the sequence.
    pub name: Option<UpdateSetNull<String>>,
    /// Description of the sequence.
    pub description: Option<UpdateSetNull<String>>,
    /// ID of asset this sequence belongs to.
    pub asset_id: Option<UpdateSetNull<i64>>,
    /// External ID of the sequence. Must be unique within the project.
    pub external_id: Option<UpdateSetNull<String>>,
    /// Custom, application specific metadata. String key -> String value.
    /// Maximum length of key is 128 bytes, up to 256 key-value pairs,
    /// up to a total size of 10000 bytes across all keys and values.
    pub metadata: Option<UpdateMap<String, String>>,
    /// ID of the data set this sequence belongs to.
    pub data_set_id: Option<UpdateSetNull<i64>>,
    /// List of columns in this sequence.
    pub columns: Option<UpdateSequenceColumns>,
}

impl IntoPatch<Patch<PatchSequence>> for Sequence {
    fn patch(self, options: &UpsertOptions) -> Patch<PatchSequence> {
        Patch::<PatchSequence> {
            id: to_idt!(self),
            update: PatchSequence {
                name: self.name.patch(options),
                description: self.description.patch(options),
                asset_id: self.asset_id.patch(options),
                external_id: self.external_id.patch(options),
                metadata: self.metadata.patch(options),
                data_set_id: self.data_set_id.patch(options),
                columns: None,
            },
        }
    }
}

impl IntoPatch<PatchSequence> for AddSequence {
    fn patch(self, options: &UpsertOptions) -> PatchSequence {
        PatchSequence {
            name: self.name.patch(options),
            description: self.description.patch(options),
            asset_id: self.asset_id.patch(options),
            external_id: self.external_id.patch(options),
            metadata: self.metadata.patch(options),
            data_set_id: self.data_set_id.patch(options),
            columns: None,
        }
    }
}

impl From<Sequence> for Patch<PatchSequence> {
    fn from(sequence: Sequence) -> Self {
        IntoPatch::<Patch<PatchSequence>>::patch(sequence, &Default::default())
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Filter on sequences.
pub struct SequenceFilter {
    /// Include sequences with this name.
    pub name: Option<String>,
    /// Filter by this (case-sensitive) prefix for the external ID.
    pub external_id_prefix: Option<String>,
    /// Filter by sequence metadata.
    pub metadata: Option<HashMap<String, String>>,
    /// Include sequences associated with one of these assets.
    pub asset_ids: Option<Vec<i64>>,
    /// Include sequences associated with an asset in the tree of one of these
    /// root assets.
    pub root_asset_ids: Option<Vec<i64>>,
    /// Include sequences associated with an asset in the subtree of one of these
    /// assets.
    pub asset_subtree_ids: Option<Vec<Identity>>,
    /// Range of timestamps for `created_time`.
    pub created_time: Option<Range<i64>>,
    /// Range of timestamps for `last_updated_time`.
    pub last_updated_time: Option<Range<i64>>,
    /// Include sequences which are tied to one of these data sets.
    pub data_set_ids: Option<Vec<Identity>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Request for filtering sequences.
pub struct SequenceFilterRequest {
    /// Simple sequences filter.
    pub filter: Option<SequenceFilter>,
    /// Advanced filter.
    pub advanced_filter: Option<AdvancedFilter>,
    /// Maximum number of sequences to return.
    pub limit: Option<i32>,
    /// Optional cursor for pagination.
    pub cursor: Option<String>,
    /// Split the result set into partitions.
    pub partition: Option<Partition>,
    /// Sort the result by these properties. The order is significant.
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
/// Fuzzy search sequences.
pub struct SequenceSearch {
    /// Fuzzy search on name.
    pub name: Option<String>,
    /// Fuzzy search on description.
    pub description: Option<String>,
    /// Searches on name and description using wildcard search on each of the words
    /// (separated by spaces). Retrieves results where at least one word must match.
    pub query: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
/// Value of a sequence row.
pub enum SequenceRowValue {
    /// String value.
    String(String),
    /// 64-bit integer value.
    Long(i64),
    /// Double precision floating point value.
    Double(f64),
    /// Null value.
    Null(()),
}

impl Default for SequenceRowValue {
    fn default() -> Self {
        SequenceRowValue::Null(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A single sequence row.
pub struct SequenceRow {
    /// Row number.
    pub row_number: i64,
    /// List of values in order of requested columns.
    pub values: Vec<SequenceRowValue>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A simple reference to a sequence column.
pub struct BasicSequenceColumn {
    /// Sequence column external ID.
    pub external_id: String,
    /// Sequence column name.
    pub name: Option<String>,
    /// Column value type.
    pub value_type: SequenceValueType,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Response when retrieving sequence rows.
pub struct RetrieveSequenceRowsResponse {
    /// Internal ID of sequence.
    pub id: i64,
    /// External ID of sequence.
    pub external_id: Option<String>,
    /// List of requested columns in sequence.
    pub columns: Vec<BasicSequenceColumn>,
    /// List of retrieved rows.
    pub rows: Vec<SequenceRow>,
    /// Cursor for pagination.
    pub next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Request to insert a list of rows into a sequence.
pub struct InsertSequenceRows {
    /// Column external IDs to insert into.
    pub columns: Vec<String>,
    /// List of rows to insert.
    pub rows: Vec<SequenceRow>,
    #[serde(flatten)]
    /// ID or external ID of sequence to insert into.
    pub id: Identity,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Request to retrieve sequence rows for a single sequence.
pub struct RetrieveSequenceRows {
    /// Optional lower bound on row numbers.
    pub start: Option<i64>,
    /// Optional upper bound on row numbers.
    pub end: Option<i64>,
    /// Maximum number of rows to return per request.
    pub limit: Option<i32>,
    /// Optional cursor for pagination.
    pub cursor: Option<String>,
    /// Columns to retrieve values from.
    pub columns: Option<Vec<String>>,
    #[serde(flatten)]
    /// ID of sequence to retrieve from.
    pub id: Identity,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Request to retrieve the sequence row with the highest row number.
pub struct RetrieveLastSequenceRow {
    /// Columns to retrieve from.
    pub columns: Option<Vec<String>>,
    /// Retrieve the row with the highest row number that is smaller than this.
    pub before: Option<i64>,
    #[serde(flatten)]
    /// ID of sequence to retrieve from.
    pub id: Identity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Request to delete a list of sequence rows from a sequence.
pub struct DeleteSequenceRows {
    /// Row numbers to delete.
    pub rows: Vec<i64>,
    #[serde(flatten)]
    /// ID of sequence to delete from.
    pub id: Identity,
}
