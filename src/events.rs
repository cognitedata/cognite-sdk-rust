use std::collections::HashMap;
use super::{
  ApiClient,
  Params,
  Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventResponseWrapper {
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
  subtype : Option<String>,
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

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Result<Vec<Event>> {
    match self.api_client.get::<EventResponseWrapper>("events", params) {
      Ok(events_response) => {
        let events = events_response.data.items;
        Ok(events)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve(&self, event_id : u64) -> Result<Event> {
    let params = None;
    match self.api_client.get::<EventResponseWrapper>(&format!("events/{}", event_id), params) {
      Ok(mut event_response) => {
        let event = event_response.data.items.pop().unwrap();
        Ok(event)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve_multiple(&self, event_ids : Vec<u64>) -> Result<Vec<Event>> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&event_ids).unwrap());
    match self.api_client.post::<EventResponseWrapper>("events/byids", &request_body){
      Ok(events_response) => {
        let events = events_response.data.items;
        Ok(events)
      },
      Err(e) => Err(e)
    }
  }

  pub fn search(&self, params : Option<Vec<Params>>) -> Result<Vec<Event>> {
    match self.api_client.get::<EventResponseWrapper>("events/search", params) {
      Ok(events_response) => {
        let events = events_response.data.items;
        Ok(events)
      },
      Err(e) => Err(e)
    }
  }

  pub fn create(&self, events : Vec<Event>) -> Event {
    unimplemented!();
  }

  pub fn update(&self, events : Vec<Event>) -> Vec<Event> {
    unimplemented!();
  }

  pub fn delete(&self, event_ids : Vec<u64>) -> () {
    unimplemented!();
  }
}

