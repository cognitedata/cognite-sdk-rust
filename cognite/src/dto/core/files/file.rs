use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{CogniteExternalId, EqIdentity, Identity, Patch, UpdateList, UpdateMap, UpdateSetNull};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileListResponse {
    pub items: Vec<FileMetadata>,
    next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
    pub external_id: Option<String>,
    pub name: String,
    pub directory: Option<String>,
    pub source: Option<String>,
    pub mime_type: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub asset_ids: Option<Vec<i64>>,
    pub data_set_id: Option<i64>,
    pub source_created_time: Option<i64>,
    pub source_modified_time: Option<i64>,
    pub security_categories: Option<Vec<i64>>,
    pub labels: Option<Vec<CogniteExternalId>>,
    pub id: i64,
    pub uploaded: bool,
    pub uploaded_time: Option<i64>,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub upload_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_created_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_modified_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_categories: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<CogniteExternalId>>,
}

impl From<&FileMetadata> for AddFile {
    fn from(file: &FileMetadata) -> AddFile {
        AddFile {
            external_id: file.external_id.clone(),
            name: file.name.clone(),
            directory: file.directory.clone(),
            source: file.source.clone(),
            mime_type: file.mime_type.clone(),
            metadata: file.metadata.clone(),
            asset_ids: file.asset_ids.clone(),
            data_set_id: file.data_set_id,
            source_created_time: file.source_created_time,
            source_modified_time: file.source_modified_time,
            security_categories: file.security_categories.clone(),
            labels: file.labels.clone(),
        }
    }
}

impl EqIdentity for AddFile {
    fn eq(&self, id: &Identity) -> bool {
        match id {
            Identity::Id { id: _ } => false,
            Identity::ExternalId { external_id } => self.external_id.as_ref() == Some(external_id),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<UpdateMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<UpdateList<i64, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_created_time: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_modified_time: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_categories: Option<UpdateList<i64, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<UpdateList<CogniteExternalId, CogniteExternalId>>,
}

impl From<&FileMetadata> for Patch<PatchFile> {
    fn from(file: &FileMetadata) -> Self {
        Self {
            id: to_idt!(file),
            update: PatchFile {
                external_id: Some(file.external_id.clone().into()),
                directory: Some(file.directory.clone().into()),
                source: Some(file.source.clone().into()),
                mime_type: Some(file.mime_type.clone().into()),
                metadata: Some(file.metadata.clone().into()),
                asset_ids: Some(file.asset_ids.clone().into()),
                source_created_time: Some(file.source_created_time.into()),
                source_modified_time: Some(file.source_modified_time.into()),
                data_set_id: Some(file.data_set_id.into()),
                security_categories: Some(file.security_categories.clone().into()),
                labels: Some(file.labels.clone().into()),
            },
        }
    }
}

impl From<&AddFile> for PatchFile {
    fn from(file: &AddFile) -> Self {
        Self {
            external_id: Some(file.external_id.clone().into()),
            directory: Some(file.directory.clone().into()),
            source: Some(file.source.clone().into()),
            mime_type: Some(file.mime_type.clone().into()),
            metadata: Some(file.metadata.clone().into()),
            asset_ids: Some(file.asset_ids.clone().into()),
            source_created_time: Some(file.source_created_time.into()),
            source_modified_time: Some(file.source_modified_time.into()),
            data_set_id: Some(file.data_set_id.into()),
            security_categories: Some(file.security_categories.clone().into()),
            labels: Some(file.labels.clone().into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileDownloadUrl {
    #[serde(flatten)]
    pub id: Identity,
    pub download_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileAggregates {
    pub count: i64,
}
