use super::{
  ApiClient
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyResponseWrapper {
  data: ApiKeyResponse,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyResponse {
  items : Vec<ApiKey>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
  pub id : u64,
  pub user_id : u64,
  pub created_time : u64,
  pub status : String,
  pub value : Option<String>
}

pub struct ApiKeys {
  api_client : ApiClient
}

impl ApiKeys {
  pub fn new(api_client : ApiClient) -> ApiKeys {
    ApiKeys {
      api_client : api_client
    }
  }

  pub fn list_all(&self) -> Vec<ApiKey> {
    unimplemented!();
  }

  pub fn create(&self, user_ids : Vec<u64>) -> Vec<ApiKey> {
    unimplemented!();
  }

  pub fn delete(&self, user_ids : Vec<u64>) -> () {
    unimplemented!();
  }
}