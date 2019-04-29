use std::collections::HashMap;
use crate::api::ApiClient;
use crate::api::params::{Params};
use crate::error::{Result};
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
  created_time : u128,
  last_updated_time : u128
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

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Result<Vec<File>> {
    match self.api_client.get::<FileResponseWrapper>("files", params){
      Ok(files_response) => {
        let files = files_response.data.items;
        Ok(files)
      },
      Err(e) => Err(e)
    }
  }

  pub fn search(&self, params : Option<Vec<Params>>) -> Result<Vec<File>> {
    match self.api_client.get::<FileResponseWrapper>("files/search", params){
      Ok(files_response) => {
        let files = files_response.data.items;
        Ok(files)
      },
      Err(e) => Err(e)
    }
  }

  pub fn upload(&self, file_stream : Vec<u8>) -> Result<FileResource> {
    unimplemented!();
  }

  pub fn delete(&self, file_ids : Vec<u64>) -> Result<()> {
    unimplemented!();
  }
}