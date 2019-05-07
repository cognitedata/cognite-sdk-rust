use crate::api::ApiClient;
use crate::error::{Result};
use crate::dto::core::files::*;

pub struct Files {
  api_client : ApiClient,
}

impl Files {
  pub fn new(api_client : ApiClient) -> Files {
    Files {
      api_client : api_client
    }
  }

  pub fn filter_all(&self, file_filter : FileFilter) -> Result<Vec<FileMetadata>> {
    let filter : Filter = Filter::new(file_filter, None, None);
    match self.api_client.post::<FileResponseWrapper>("files/list", &serde_json::to_string(&filter).unwrap()){
      Ok(files_response) => {
        let files = files_response.data.items;
        Ok(files)
      },
      Err(e) => Err(e)
    }
  }

  pub fn upload(&self, _file_stream : Vec<u8>) -> Result<FileMetadata> {
    unimplemented!();
  }

  pub fn retrieve_metadata(&self, file_ids : &[u64]) -> Result<Vec<FileMetadata>> {
    let id_list : Vec<FileId> = file_ids.iter().map(| f_id | FileId::from(*f_id)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&id_list).unwrap());
    match self.api_client.post::<FileResponseWrapper>("files/byids", &request_body){
      Ok(files_response) => {
        let files = files_response.data.items;
        Ok(files)
      },
      Err(e) => Err(e)
    }
  }

  pub fn search(&self, file_filter : FileFilter, file_search : FileSearch) -> Result<Vec<FileMetadata>> {
    let filter : Search = Search::new(file_filter, file_search, None);
    match self.api_client.post::<FileResponseWrapper>("files/search", &serde_json::to_string(&filter).unwrap()){
      Ok(files_response) => {
        let files = files_response.data.items;
        Ok(files)
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, file_ids : Vec<u64>) -> Result<()> {
    let id_list : Vec<FileId> = file_ids.iter().map(| a_id | FileId::from(*a_id)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&id_list).unwrap());
    match self.api_client.post::<::serde_json::Value>("files/delete", &request_body){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
  
  pub fn download(&self, file_ids : Vec<u64>) -> Result<Vec<FileLink>> {
    let id_list : Vec<FileId> = file_ids.iter().map(| a_id | FileId::from(*a_id)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&id_list).unwrap());
    match self.api_client.post::<FileLinkResponseWrapper>("files/download", &request_body){
      Ok(file_links_response) => {
        let file_links = file_links_response.data.items;
        Ok(file_links)
      },
      Err(e) => Err(e)
    }
  }

  pub fn update(&self, _file_ids : Vec<u64>) -> Result<()> {
    unimplemented!();
  }
}