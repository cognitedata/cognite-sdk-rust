use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::{CogniteExternalId, EqIdentity, Identity, Patch, UpdateList, UpdateMap, UpdateSetNull};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileListResponse {
    pub items: Vec<FileMetadata>,
    next_cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddFile {
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
}

impl From<FileMetadata> for AddFile {
    fn from(file: FileMetadata) -> AddFile {
        AddFile {
            external_id: file.external_id,
            name: file.name,
            directory: file.directory,
            source: file.source,
            mime_type: file.mime_type,
            metadata: file.metadata,
            asset_ids: file.asset_ids,
            data_set_id: file.data_set_id,
            source_created_time: file.source_created_time,
            source_modified_time: file.source_modified_time,
            security_categories: file.security_categories,
            labels: file.labels,
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchFile {
    pub external_id: Option<UpdateSetNull<String>>,
    pub directory: Option<UpdateSetNull<String>>,
    pub source: Option<UpdateSetNull<String>>,
    pub mime_type: Option<UpdateSetNull<String>>,
    pub metadata: Option<UpdateMap<String, String>>,
    pub asset_ids: Option<UpdateList<i64, i64>>,
    pub source_created_time: Option<UpdateSetNull<i64>>,
    pub source_modified_time: Option<UpdateSetNull<i64>>,
    pub data_set_id: Option<UpdateSetNull<i64>>,
    pub security_categories: Option<UpdateList<i64, i64>>,
    pub labels: Option<UpdateList<CogniteExternalId, CogniteExternalId>>,
}

impl From<FileMetadata> for Patch<PatchFile> {
    fn from(file: FileMetadata) -> Self {
        Self {
            id: to_idt!(file),
            update: PatchFile {
                external_id: Some(file.external_id.into()),
                directory: Some(file.directory.into()),
                source: Some(file.source.into()),
                mime_type: Some(file.mime_type.into()),
                metadata: Some(file.metadata.into()),
                asset_ids: Some(file.asset_ids.into()),
                source_created_time: Some(file.source_created_time.into()),
                source_modified_time: Some(file.source_modified_time.into()),
                data_set_id: Some(file.data_set_id.into()),
                security_categories: Some(file.security_categories.into()),
                labels: Some(file.labels.into()),
            },
        }
    }
}

impl From<AddFile> for PatchFile {
    fn from(file: AddFile) -> Self {
        Self {
            external_id: Some(file.external_id.into()),
            directory: Some(file.directory.into()),
            source: Some(file.source.into()),
            mime_type: Some(file.mime_type.into()),
            metadata: Some(file.metadata.into()),
            asset_ids: Some(file.asset_ids.into()),
            source_created_time: Some(file.source_created_time.into()),
            source_modified_time: Some(file.source_modified_time.into()),
            data_set_id: Some(file.data_set_id.into()),
            security_categories: Some(file.security_categories.into()),
            labels: Some(file.labels.into()),
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
