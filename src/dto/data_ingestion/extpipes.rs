use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Identity, Patch, Range, UpdateList, UpdateMap, UpdateSet};

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtPipeRawTable {
    pub db_name: String,
    pub table_name: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtPipeContact {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub send_notification: Option<bool>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddExtPipe {
    pub external_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub data_set_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_tables: Option<Vec<ExtPipeRawTable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<ExtPipeContact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
}

impl From<&ExtPipe> for AddExtPipe {
    fn from(pipe: &ExtPipe) -> Self {
        AddExtPipe {
            external_id: pipe.external_id.clone(),
            name: pipe.name.clone(),
            description: pipe.description.clone(),
            data_set_id: pipe.data_set_id,
            raw_tables: pipe.raw_tables.clone(),
            schedule: pipe.schedule.clone(),
            contacts: pipe.contacts.clone(),
            metadata: pipe.metadata.clone(),
            source: pipe.source.clone(),
            documentation: pipe.documentation.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchExtPipe {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<UpdateSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UpdateSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<UpdateSet<Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<UpdateSet<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<UpdateSet<Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_tables: Option<UpdateList<ExtPipeRawTable, ExtPipeRawTable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<UpdateList<ExtPipeContact, ExtPipeContact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<UpdateMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<UpdateSet<Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<UpdateSet<Option<String>>>,
}

impl From<&ExtPipe> for Patch<PatchExtPipe> {
    fn from(pipe: &ExtPipe) -> Self {
        Patch::<PatchExtPipe> {
            id: Identity::ExternalId {
                external_id: pipe.external_id.clone(),
            },
            update: PatchExtPipe {
                external_id: Some(pipe.external_id.clone().into()),
                name: Some(pipe.name.clone().into()),
                description: Some(pipe.description.clone().into()),
                data_set_id: Some(pipe.data_set_id.into()),
                schedule: Some(pipe.schedule.clone().into()),
                raw_tables: Some(pipe.raw_tables.clone().into()),
                contacts: Some(pipe.contacts.clone().into()),
                metadata: Some(pipe.metadata.clone().into()),
                source: Some(pipe.source.clone().into()),
                documentation: Some(pipe.documentation.clone().into()),
            },
        }
    }
}

impl From<&AddExtPipe> for PatchExtPipe {
    fn from(pipe: &AddExtPipe) -> Self {
        PatchExtPipe {
            external_id: Some(pipe.external_id.clone().into()),
            name: Some(pipe.name.clone().into()),
            description: Some(pipe.description.clone().into()),
            data_set_id: Some(pipe.data_set_id.into()),
            schedule: Some(pipe.schedule.clone().into()),
            raw_tables: Some(pipe.raw_tables.clone().into()),
            contacts: Some(pipe.contacts.clone().into()),
            metadata: Some(pipe.metadata.clone().into()),
            source: Some(pipe.source.clone().into()),
            documentation: Some(pipe.documentation.clone().into()),
        }
    }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Default)]
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtPipeRun {
    pub id: i64,
    pub status: ExtPipeRunStatus,
    pub message: Option<String>,
    pub created_time: i64,
    pub external_id: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddExtPipeRun {
    pub status: ExtPipeRunStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    pub external_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtPipeStringFilter {
    pub substring: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtPipeRunFilter {
    pub external_id: String,
    pub statuses: Option<Vec<ExtPipeRunStatus>>,
    pub created_time: Option<Range<i64>>,
    pub message: Option<ExtPipeStringFilter>,
}
