use crate::dto::params::{Params};
use crate::error::{Result};
use crate::api::ApiClient;
use crate::dto::iam::api_key::*;
use crate::dto::items::Items;

pub struct ApiKeys {
  api_client : ApiClient
}

impl ApiKeys {
  pub fn new(api_client : ApiClient) -> ApiKeys {
    ApiKeys {
      api_client : api_client
    }
  }

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Result<Vec<ApiKey>> {
    match self.api_client.get_with_params::<ApiKeyListResponse>("apikeys", params){
      Ok(api_keys_response) => {
        let api_keys = api_keys_response.items;
        Ok(api_keys)
      },
      Err(e) => Err(e)
    }
  }

  pub fn create(&self, service_account_ids : &[u64]) -> Result<Vec<ApiKey>> {
    let service_account_id_items = Items::from(service_account_ids);
    match self.api_client.post("apikeys", &service_account_id_items){
      Ok(result) => {
        let api_keys_response : ApiKeyListResponse = result;
        let api_keys = api_keys_response.items;
        Ok(api_keys)
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, service_account_ids : &[u64]) -> Result<()> {
    let service_account_id_items = Items::from(service_account_ids);
    match self.api_client.post::<::serde_json::Value, Items>("apikeys/delete", &service_account_id_items){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}