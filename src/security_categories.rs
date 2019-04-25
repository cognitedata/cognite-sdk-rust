use super::{
  ApiClient,
  Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecurityCategorieResponseWrapper {
  data: SecurityCategorieResponse,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecurityCategorieResponse {
  items : Vec<SecurityCategorie>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecurityCategorie {
  pub name : String,
  pub id : u64,
}

pub struct SecurityCategories {
  api_client : ApiClient
}

impl SecurityCategories {
  pub fn new(api_client : ApiClient) -> SecurityCategories {
    SecurityCategories {
      api_client : api_client
    }
  }

  pub fn list_all(&self) -> Result<Vec<SecurityCategorie>> {
    unimplemented!();
  }

  pub fn create(&self, security_categorie_ids : Vec<u64>) -> Result<Vec<SecurityCategorie>> {
    unimplemented!();
  }

  pub fn delete(&self, security_categorie_ids : Vec<u64>) -> Result<()> {
    unimplemented!();
  }
}