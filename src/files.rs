use std::collections::HashMap;
use super::{
  ApiClient,
  Params,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileResponseWrapper {
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
  source_id : Option<String>,
  file_type : String,
  metadata: HashMap<String, String>,
  asset_ids  : Option<Vec<u64>>,
  uploaded  : bool,
  uploaded_at  : u64,
  created_time : u64,
  last_updated_time : u64
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileResource {
  file_id : u64,
  upload_url : String,
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

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Vec<File> {
    let files_response_json = self.api_client.get("files", params).unwrap();
    let files_response : FileResponseWrapper = serde_json::from_str(&files_response_json).unwrap();
    let files = files_response.data.items;
    files
  }

  pub fn search(&self, params : Option<Vec<Params>>) -> Vec<File> {
    let files_response_json = self.api_client.get("files/search", params).unwrap();
    let files_response : FileResponseWrapper = serde_json::from_str(&files_response_json).unwrap();
    let files = files_response.data.items;
    files
  }

  pub fn upload(&self, file_stream : Vec<u8>) -> FileResource {
    unimplemented!();
  }

  pub fn delete(&self, file_ids : Vec<u64>) -> () {
    unimplemented!();
  }
}