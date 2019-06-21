use crate::dto::params::{Params};
use crate::error::{Result};
use crate::api::ApiClient;
use crate::dto::iam::group::*;
use crate::dto::iam::service_account::*;
use crate::dto::items::Items;

pub struct Groups {
  api_client : ApiClient
}

impl Groups {
  pub fn new(api_client : ApiClient) -> Groups {
    Groups {
      api_client : api_client
    }
  }

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Result<Vec<Group>> {
    match self.api_client.get_with_params::<GroupListResponse>("groups", params){
      Ok(groups_response) => {
        let groups = groups_response.items;
        Ok(groups)
      },
      Err(e) => Err(e)
    }
  }

  pub fn create(&self, groups : &[Group]) -> Result<Vec<Group>> {
    let groups_items = Items::from(groups);
    match self.api_client.post("groups", &groups_items){
      Ok(result) => {
        let groups_response : GroupListResponse = result;
        let groups = groups_response.items;
        Ok(groups)
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, groups_ids : &[u64]) -> Result<()> {
    let groups_id_items = Items::from(groups_ids);
    match self.api_client.post::<::serde_json::Value, Items>("groups/delete", &groups_id_items){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }

  pub fn list_service_accounts(&self, group_id : u64) -> Result<Vec<ServiceAccount>> {
    match self.api_client.get::<ServiceAccountListResponse>(&format!("groups/{}/serviceaccounts", group_id)){
      Ok(service_accounts_response) => {
        let service_accounts = service_accounts_response.items;
        Ok(service_accounts)
      },
      Err(e) => Err(e)
    }
  }

  pub fn add_service_accounts(&self, group_id : u64, service_account_ids : &[u64]) -> Result<()> {
    let id_items = Items::from(service_account_ids);
    match self.api_client.post::<::serde_json::Value, Items>(&format!("groups/{}/serviceaccounts", group_id), &id_items){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }

  pub fn remove_service_accounts(&self, group_id : u64, service_account_ids : &[u64]) -> Result<()> {
    let id_items = Items::from(service_account_ids);
    match self.api_client.post::<::serde_json::Value, Items>(&format!("groups/{}/serviceaccounts/remove", group_id), &id_items){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}