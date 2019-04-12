use std::collections::HashMap;
use super::{ApiClient};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileResponse {
  data : FileListResponse
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileListResponse {
  items : Vec<File>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct File {
  id : u64,
  file_name : String,
  directory : String,
  source : String,
  source_id : String,
  file_type : String,
  metadata: HashMap<String, String>,
  asset_ids  : Vec<u64>,
  uploaded  : bool,
  uploaded_at  : u128,
  created_time : u128,
  last_updated_time : u128
}

pub struct Files {
  api_client : ApiClient,
}

impl Files {
  pub fn new(api_client : ApiClient) -> Files {
    Files {
      api_client : api_client
    }
  }
}