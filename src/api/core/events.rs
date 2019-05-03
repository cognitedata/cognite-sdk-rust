use std::collections::HashMap;
use crate::api::ApiClient;
use crate::dto::params::{Params};
use crate::error::{Result};
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

impl Event {
  pub fn new(start_time : Option<u128>,
          end_time : Option<u128>,
          description : Option<String>,
          r#type : String,
          subtype : Option<String>,
          metadata : Option<HashMap<String, String>>,
          asset_ids : Vec<u64>,
          source : String,
          source_id : String) -> Event {
    Event {
      id : 0,
      start_time : start_time,
      end_time : end_time,
      description : description,
      r#type : r#type,
      subtype : subtype,
      metadata : metadata,
      asset_ids : asset_ids,
      source : source,
      source_id : source_id,
      created_time : 0,
      last_updated_time : 0,
    }
  }
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
    match self.api_client.get_with_params::<EventResponseWrapper>("events", params) {
      Ok(events_response) => {
        let events = events_response.data.items;
        Ok(events)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve(&self, event_id : u64) -> Result<Event> {
    let params = None;
    match self.api_client.get_with_params::<EventResponseWrapper>(&format!("events/{}", event_id), params) {
      Ok(mut event_response) => {
        let event = event_response.data.items.pop().unwrap();
        Ok(event)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve_multiple(&self, event_ids : &[u64]) -> Result<Vec<Event>> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(event_ids).unwrap());
    match self.api_client.post::<EventResponseWrapper>("events/byids", &request_body){
      Ok(events_response) => {
        let events = events_response.data.items;
        Ok(events)
      },
      Err(e) => Err(e)
    }
  }

  pub fn search(&self, params : Option<Vec<Params>>) -> Result<Vec<Event>> {
    match self.api_client.get_with_params::<EventResponseWrapper>("events/search", params) {
      Ok(events_response) => {
        let events = events_response.data.items;
        Ok(events)
      },
      Err(e) => Err(e)
    }
  }

  pub fn create(&self, events : &[Event]) -> Result<Vec<Event>> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(events).unwrap());
    match self.api_client.post::<EventResponseWrapper>("events", &request_body){
      Ok(events_response) => {
        let events = events_response.data.items;
        Ok(events)
      },
      Err(e) => Err(e)
    }
  }

  pub fn update(&self, events : &[Event]) -> Result<()> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(events).unwrap());
    match self.api_client.post::<::serde_json::Value>("events/update", &request_body){
      Ok(_) => Ok(()),
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, event_ids : &[u64]) -> Result<()> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(event_ids).unwrap());
    match self.api_client.post::<::serde_json::Value>("events/delete", &request_body){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}

