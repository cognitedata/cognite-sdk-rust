use super::{
  ApiClient,
  Params,
  Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserResponseWrapper {
  data: UserResponse,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
  items : Vec<User>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
  pub unique_name : String,
  pub groups : Vec<u64>,
  pub id : u64,
  pub is_deleted : bool,
  pub deleted_time : i64
}

pub struct Users {
  api_client : ApiClient
}

impl Users {
  pub fn new(api_client : ApiClient) -> Users {
    Users {
      api_client : api_client
    }
  }

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Result<Vec<User>> {
    match self.api_client.get::<UserResponseWrapper>("users", params){
      Ok(users_response) => {
        let users = users_response.data.items;
        Ok(users)
      },
      Err(e) => Err(e)
    }
  }

  pub fn create(&self, user_ids : Vec<u64>) -> Result<Vec<User>> {
    unimplemented!();
  }

  pub fn delete(&self, user_ids : Vec<u64>) -> Result<()> {
    unimplemented!();
  }
}