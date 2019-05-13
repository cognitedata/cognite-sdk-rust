use crate::api::ApiClient;
use crate::dto::iam::api_key::*;

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

  pub fn create(&self, user_ids : &[u64]) -> Vec<ApiKey> {
    unimplemented!();
  }

  pub fn delete(&self, user_ids : &[u64]) -> () {
    unimplemented!();
  }
}