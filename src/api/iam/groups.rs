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
    let groups_response : GroupListResponse = self.api_client.get_with_params("groups", params)?;
    Ok(groups_response.items)
  }

  pub fn create(&self, groups : &[Group]) -> Result<Vec<Group>> {
    let groups_items = Items::from(groups);
    let groups_response : GroupListResponse = self.api_client.post("groups", &groups_items)?;
    Ok(groups_response.items)
  }

  pub fn delete(&self, groups_ids : &[u64]) -> Result<()> {
    let groups_id_items = Items::from(groups_ids);
    self.api_client.post::<::serde_json::Value, Items>("groups/delete", &groups_id_items)?;
    Ok(())
  }

  pub fn list_service_accounts(&self, group_id : u64) -> Result<Vec<ServiceAccount>> {
    let service_accounts_response : ServiceAccountListResponse = self.api_client.get(&format!("groups/{}/serviceaccounts", group_id))?;
    Ok(service_accounts_response.items)
  }

  pub fn add_service_accounts(&self, group_id : u64, service_account_ids : &[u64]) -> Result<()> {
    let id_items = Items::from(service_account_ids);
    self.api_client.post::<::serde_json::Value, Items>(&format!("groups/{}/serviceaccounts", group_id), &id_items)?;
    Ok(())
  }

  pub fn remove_service_accounts(&self, group_id : u64, service_account_ids : &[u64]) -> Result<()> {
    let id_items = Items::from(service_account_ids);
    self.api_client.post::<::serde_json::Value, Items>(&format!("groups/{}/serviceaccounts/remove", group_id), &id_items)?;
    Ok(())
  }
}