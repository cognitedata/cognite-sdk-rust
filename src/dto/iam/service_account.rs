use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountListResponse {
  pub items : Vec<ServiceAccount>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccount {
  pub name : String,
  pub groups : Vec<u64>,
  #[serde(skip_serializing)]
  pub id : u64,
  #[serde(skip_serializing)]
  pub is_deleted : bool,
  #[serde(skip_serializing)]
  pub deleted_time : Option<i64>
}

impl ServiceAccount {
  pub fn new(name : &str, groups : &[u64]) -> ServiceAccount {
    ServiceAccount {
      name : String::from(name),
      groups : groups.to_vec(),
      id : 0,
      is_deleted : false,
      deleted_time : None
    }
  }
}