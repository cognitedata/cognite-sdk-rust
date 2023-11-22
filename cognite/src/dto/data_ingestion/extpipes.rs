use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Identity, IntoPatch, IntoPatchItem, Patch, Range, UpdateList, UpdateMap, UpdateSet};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtPipeRawTable {
    pub db_name: String,
    pub table_name: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtPipeContact {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub send_notification: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtPipe {
    pub id: i64,
    pub external_id: String,
    pub name: String,
    pub description: Option<String>,
    pub data_set_id: i64,
    pub raw_tables: Option<Vec<ExtPipeRawTable>>,
    pub schedule: Option<String>,
    pub contacts: Option<Vec<ExtPipeContact>>,
    pub metadata: Option<HashMap<String, String>>,
    pub source: Option<String>,
    pub documentation: Option<String>,
    pub last_success: Option<i64>,
    pub last_failure: Option<i64>,
    pub last_message: Option<String>,
    pub last_seen: Option<i64>,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub created_by: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddExtPipe {
    pub external_id: String,
    pub name: String,
    pub description: Option<String>,
    pub data_set_id: i64,
    pub raw_tables: Option<Vec<ExtPipeRawTable>>,
    pub schedule: Option<String>,
    pub contacts: Option<Vec<ExtPipeContact>>,
    pub metadata: Option<HashMap<String, String>>,
    pub source: Option<String>,
    pub documentation: Option<String>,
}

impl From<ExtPipe> for AddExtPipe {
    fn from(pipe: ExtPipe) -> Self {
        AddExtPipe {
            external_id: pipe.external_id,
            name: pipe.name,
            description: pipe.description,
            data_set_id: pipe.data_set_id,
            raw_tables: pipe.raw_tables,
            schedule: pipe.schedule,
            contacts: pipe.contacts,
            metadata: pipe.metadata,
            source: pipe.source,
            documentation: pipe.documentation,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PatchExtPipe {
    pub external_id: Option<UpdateSet<String>>,
    pub name: Option<UpdateSet<String>>,
    pub description: Option<UpdateSet<Option<String>>>,
    pub data_set_id: Option<UpdateSet<i64>>,
    pub schedule: Option<UpdateSet<Option<String>>>,
    pub raw_tables: Option<UpdateList<ExtPipeRawTable, ExtPipeRawTable>>,
    pub contacts: Option<UpdateList<ExtPipeContact, ExtPipeContact>>,
    pub metadata: Option<UpdateMap<String, String>>,
    pub source: Option<UpdateSet<Option<String>>>,
    pub documentation: Option<UpdateSet<Option<String>>>,
}

impl IntoPatch<Patch<PatchExtPipe>> for ExtPipe {
    fn patch(self, ignore_nulls: bool) -> Patch<PatchExtPipe> {
        Patch::<PatchExtPipe> {
            id: Identity::ExternalId {
                external_id: self.external_id,
            },
            update: PatchExtPipe {
                external_id: None,
                name: self.name.patch(ignore_nulls),
                description: self.description.patch(ignore_nulls),
                data_set_id: self.data_set_id.patch(ignore_nulls),
                schedule: self.schedule.patch(ignore_nulls),
                raw_tables: self.raw_tables.patch(ignore_nulls),
                contacts: self.contacts.patch(ignore_nulls),
                metadata: self.metadata.patch(ignore_nulls),
                source: self.source.patch(ignore_nulls),
                documentation: self.documentation.patch(ignore_nulls),
            },
        }
    }
}

impl IntoPatch<PatchExtPipe> for AddExtPipe {
    fn patch(self, ignore_nulls: bool) -> PatchExtPipe {
        PatchExtPipe {
            external_id: self.external_id.patch(ignore_nulls),
            name: self.name.patch(ignore_nulls),
            description: self.description.patch(ignore_nulls),
            data_set_id: self.data_set_id.patch(ignore_nulls),
            schedule: self.schedule.patch(ignore_nulls),
            raw_tables: self.raw_tables.patch(ignore_nulls),
            contacts: self.contacts.patch(ignore_nulls),
            metadata: self.metadata.patch(ignore_nulls),
            source: self.source.patch(ignore_nulls),
            documentation: self.documentation.patch(ignore_nulls),
        }
    }
}

impl From<ExtPipe> for Patch<PatchExtPipe> {
    fn from(sequence: ExtPipe) -> Self {
        IntoPatch::<Patch<PatchExtPipe>>::patch(sequence, false)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ExtPipeRunStatus {
    Success,
    Failure,
    Seen,
}

impl Default for ExtPipeRunStatus {
    fn default() -> Self {
        Self::Seen
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ExtPipeFilter {
    pub external_id_prefix: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub data_set_ids: Option<Vec<Identity>>,
    pub schedule: Option<String>,
    pub contacts: Option<Vec<ExtPipeContact>>,
    pub raw_tables: Option<Vec<ExtPipeRawTable>>,
    pub metadata: Option<HashMap<String, String>>,
    pub source: Option<String>,
    pub documentation: Option<String>,
    pub created_by: Option<String>,
    pub created_time: Option<Range<i64>>,
    pub last_updated_time: Option<Range<i64>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtPipeRun {
    pub id: i64,
    pub status: ExtPipeRunStatus,
    pub message: Option<String>,
    pub created_time: i64,
    pub external_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddExtPipeRun {
    pub status: ExtPipeRunStatus,
    pub message: Option<String>,
    pub created_time: Option<i64>,
    pub external_id: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtPipeStringFilter {
    pub substring: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtPipeRunFilter {
    pub external_id: String,
    pub statuses: Option<Vec<ExtPipeRunStatus>>,
    pub created_time: Option<Range<i64>>,
    pub message: Option<ExtPipeStringFilter>,
}
