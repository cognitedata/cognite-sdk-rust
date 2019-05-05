use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventResponseWrapper {
  pub data : EventListResponse
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventListResponse {
  pub items : Vec<Event>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
  pub id : u64,
  pub start_time : Option<u128>,
  pub end_time : Option<u128>,
  pub description : Option<String>,
  pub r#type : String,
  pub subtype : Option<String>,
  pub metadata : Option<HashMap<String, String>>,
  pub asset_ids : Vec<u64>,
  pub source : String,
  pub source_id : String,
  pub created_time : u128,
  pub last_updated_time : u128,
}
