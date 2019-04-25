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
  #[serde(skip_serializing)]
  pub id : u64,
  #[serde(skip_serializing)]
  pub is_deleted : bool,
  #[serde(skip_serializing)]
  pub deleted_time : Option<i64>
}

impl User {
  pub fn new(unique_name : &str, groups : Vec<u64>) -> User {
    User {
      unique_name : String::from(unique_name),
      groups : groups,
      id : 0,
      is_deleted : false,
      deleted_time : None
    }
  }
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

  pub fn create(&self, users : Vec<User>) -> Result<Vec<User>> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&users).unwrap());
    match self.api_client.post::<UserResponseWrapper>("users", &request_body){
      Ok(assets_response) => {
        let assets = assets_response.data.items;
        Ok(assets)
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, user_ids : Vec<u64>) -> Result<()> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&user_ids).unwrap());
    match self.api_client.post::<::serde_json::Value>("users/delete", &request_body){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}