use std::collections::HashMap;
use super::{
  ApiClient,
  Params,
};
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
  previous_cursor : Option<String>,
  next_cursor : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
  start_time : Option<u64>,
  end_time : Option<u64>,
  description : Option<String>,
  r#type : String,
  subtype : String,
  metadata : HashMap<String, String>,
  asset_ids : Vec<u64>,
  source : String,
  source_id : String,
  created_time : u64,
  last_updated_time : u64,
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

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Vec<Event> {
    let events_response_json = self.api_client.get("events", params).unwrap();
    let events_response : EventResponse = serde_json::from_str(&events_response_json).unwrap();
    let events = events_response.data.items;
    events
  }

  pub fn search(&self, params : Option<Vec<Params>>) -> Vec<Event> {
    let events_response_json = self.api_client.get("events/search", params).unwrap();
    let events_response : EventResponse = serde_json::from_str(&events_response_json).unwrap();
    let events = events_response.data.items;
    events
  }
}

