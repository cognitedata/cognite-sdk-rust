use crate::api::ApiClient;
use crate::dto::params::{Params};
use crate::error::{Result};
use crate::dto::iam::security_category::*;

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
    match self.api_client.get_with_params::<SecurityCategorieResponseWrapper>("securitycategories", params) {
      Ok(security_categories_response) => {
        Ok(security_categories_response.data.items)
      }
      Err(e) => Err(e)
    }
  }

  pub fn create(&self, security_category_names : &[SecurityCategory]) -> Result<Vec<SecurityCategory>> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(security_category_names).unwrap());
    match self.api_client.post::<SecurityCategorieResponseWrapper>("securitycategories", &request_body){
      Ok(security_categories_response) => {
        let security_categories = security_categories_response.data.items;
        Ok(security_categories)
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, security_category_ids : &[u64]) -> Result<()> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(security_category_ids).unwrap());
    match self.api_client.post::<::serde_json::Value>("securitycategories/delete", &request_body){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}