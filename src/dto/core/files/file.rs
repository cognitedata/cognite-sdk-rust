use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileListResponse {
  pub items : Vec<FileMetadata>,
  next_cursor : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
  pub id : u64,
  pub external_id : Option<String>,
  pub name : String,
  pub source : Option<String>,
  pub mime_type : Option<String>,
  pub metadata: HashMap<String, String>,
  pub asset_ids : Option<Vec<u64>>,
  pub uploaded : Option<bool>,
  pub uploaded_time : Option<u64>,
  pub created_time : u128,
  pub last_updated_time : u128
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileLinkListResponse {
  pub items : Vec<FileLink>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileLink {
  pub id : u64,
  pub link : String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileId {
  id : u64
}

impl From<&FileMetadata> for FileId {
  fn from(file_metadata : &FileMetadata) -> FileId {
    FileId {
      id : file_metadata.id
    }
  }
}

impl From<u64> for FileId {
  fn from(file_id : u64) -> FileId {
    FileId {
      id : file_id
    }
  }
}
