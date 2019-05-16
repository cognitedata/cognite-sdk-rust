use crate::api::ApiClient;
use crate::dto::params::{Params};
use crate::error::{Result};
use crate::dto::iam::security_category::*;
use crate::dto::items::Items;

pub struct SecurityCategories {
  api_client : ApiClient
}

impl SecurityCategories {
  pub fn new(api_client : ApiClient) -> SecurityCategories {
    SecurityCategories {
      api_client : api_client
    }
  }

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Result<Vec<SecurityCategory>> {
    match self.api_client.get_with_params::<SecurityCategoryListResponse>("securitycategories", params) {
      Ok(security_categories_response) => {
        Ok(security_categories_response.items)
      }
      Err(e) => Err(e)
    }
  }

  pub fn create(&self, security_category_names : &[SecurityCategory]) -> Result<Vec<SecurityCategory>> {
    let security_category_name_items = Items::from(security_category_names);
    match self.api_client.post("securitycategories", &security_category_name_items){
      Ok(result) => {
        let security_categories_response : SecurityCategoryListResponse = result;
        let security_categories = security_categories_response.items;
        Ok(security_categories)
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, security_category_ids : &[u64]) -> Result<()> {
    let id_items = Items::from(security_category_ids);
    match self.api_client.post::<::serde_json::Value, Items>("securitycategories/delete", &id_items){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}