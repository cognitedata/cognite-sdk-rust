use std::collections::HashMap;
use super::{ApiClient};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventResponse {
  data : EventListResponse
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventListResponse {
  items : Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
  start_time : u128,
  end_time : u128,
  description : String,
  r#type : String,
  subtype : String,
  metadata : HashMap<String, String>,
  asset_ids : Vec<u64>,
  source : String,
  source_id : String,
  created_time : u128,
  last_updated_time : u128,
}

pub struct Events {
  api_client : ApiClient,
}

impl Events {
  pub fn new(api_client : ApiClient) -> Events {
    Events {
      api_client : api_client
    }
  }
}
