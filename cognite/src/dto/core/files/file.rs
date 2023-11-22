use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::{
    CogniteExternalId, EqIdentity, Identity, IntoPatch, IntoPatchItem, Patch, UpdateList,
    UpdateMap, UpdateSetNull,
};

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
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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

impl IntoPatch<Patch<PatchFile>> for FileMetadata {
    fn patch(self, ignore_nulls: bool) -> Patch<PatchFile> {
        Patch::<PatchFile> {
            id: to_idt!(self),
            update: PatchFile {
                external_id: self.external_id.patch(ignore_nulls),
                directory: self.directory.patch(ignore_nulls),
                source: self.source.patch(ignore_nulls),
                mime_type: self.mime_type.patch(ignore_nulls),
                metadata: self.metadata.patch(ignore_nulls),
                asset_ids: self.asset_ids.patch(ignore_nulls),
                source_created_time: self.source_created_time.patch(ignore_nulls),
                source_modified_time: self.source_modified_time.patch(ignore_nulls),
                data_set_id: self.data_set_id.patch(ignore_nulls),
                security_categories: self.security_categories.patch(ignore_nulls),
                labels: self.labels.patch(ignore_nulls),
            },
        }
    }
}

impl IntoPatch<PatchFile> for AddFile {
    fn patch(self, ignore_nulls: bool) -> PatchFile {
        PatchFile {
            external_id: self.external_id.patch(ignore_nulls),
            directory: self.directory.patch(ignore_nulls),
            source: self.source.patch(ignore_nulls),
            mime_type: self.mime_type.patch(ignore_nulls),
            metadata: self.metadata.patch(ignore_nulls),
            asset_ids: self.asset_ids.patch(ignore_nulls),
            source_created_time: self.source_created_time.patch(ignore_nulls),
            source_modified_time: self.source_modified_time.patch(ignore_nulls),
            data_set_id: self.data_set_id.patch(ignore_nulls),
            security_categories: self.security_categories.patch(ignore_nulls),
            labels: self.labels.patch(ignore_nulls),
        }
    }
}

impl From<FileMetadata> for Patch<PatchFile> {
    fn from(file: FileMetadata) -> Self {
        IntoPatch::<Patch<PatchFile>>::patch(file, false)
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
