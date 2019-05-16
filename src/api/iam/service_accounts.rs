use crate::api::ApiClient;
use crate::dto::params::{Params};
use crate::error::{Result};
use crate::dto::iam::service_account::*;
use crate::dto::items::Items;

pub struct ServiceAccounts {
  api_client : ApiClient
}

impl ServiceAccounts {
  pub fn new(api_client : ApiClient) -> ServiceAccounts {
    ServiceAccounts {
      api_client : api_client
    }
  }

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Result<Vec<ServiceAccount>> {
    match self.api_client.get_with_params::<ServiceAccountListResponse>("serviceaccounts", params){
      Ok(service_accounts_response) => {
        let service_accounts = service_accounts_response.items;
        Ok(service_accounts)
      },
      Err(e) => Err(e)
    }
  }

  pub fn create(&self, service_accounts : &[ServiceAccount]) -> Result<Vec<ServiceAccount>> {
    let service_accounts_items = Items::from(service_accounts);
    match self.api_client.post("serviceaccounts", &service_accounts_items){
      Ok(result) => {
        let service_accounts_response : ServiceAccountListResponse = result;
        let service_accounts = service_accounts_response.items;
        Ok(service_accounts)
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, user_ids : &[u64]) -> Result<()> {
    let id_items = Items::from(user_ids);
    match self.api_client.post::<::serde_json::Value, Items>("serviceaccounts/delete", &id_items){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}