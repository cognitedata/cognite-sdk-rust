use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileListResponse {
    pub items: Vec<FileMetadata>,
    next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
    pub external_id: Option<String>,
    pub name: String,
    pub source: Option<String>,
    pub mime_type: Option<String>,
    pub metadata: HashMap<String, String>,
    pub asset_ids: Option<Vec<u64>>,
    pub source_created_time: i64,
    pub source_modified_time: i64,
    pub id: u64,
    pub uploaded: Option<bool>,
    pub uploaded_time: Option<i64>,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub uploaded_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileLinkListResponse {
    pub items: Vec<FileLink>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileLink {
    pub id: u64,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileId {
    id: u64,
}

impl From<&FileMetadata> for FileId {
    fn from(file_metadata: &FileMetadata) -> FileId {
        FileId {
            id: file_metadata.id,
        }
    }
}

impl From<u64> for FileId {
    fn from(file_id: u64) -> FileId {
        FileId { id: file_id }
    }
}
