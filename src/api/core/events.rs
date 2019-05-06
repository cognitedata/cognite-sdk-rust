use std::collections::HashMap;
use crate::api::ApiClient;
use crate::error::{Result};
use crate::dto::params::{Params};
use crate::dto::core::event::*;

pub struct Events {
  api_client : ApiClient,
}

impl Events {
  pub fn new(api_client : ApiClient) -> Events {
    Events {
      api_client : api_client
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

  pub fn filter_all(&self, event_filter : EventFilter) -> Result<Vec<Event>> {
    let filter : Filter = Filter::new(event_filter, None, None);
    match self.api_client.post::<EventResponseWrapper>("events/list", &serde_json::to_string(&filter).unwrap()) {
      Ok(events_response) => {
        let events = events_response.data.items;
        Ok(events)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve_single(&self, event_id : u64) -> Result<Event> {
    match self.api_client.get::<EventResponseWrapper>(&format!("events/{}", event_id)) {
      Ok(mut event_response) => {
        let event = event_response.data.items.pop().unwrap();
        Ok(event)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve(&self, event_ids : &[u64]) -> Result<Vec<Event>> {
    let id_list : Vec<EventId> = event_ids.iter().map(| e_id | EventId::from(*e_id)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&id_list).unwrap());
    match self.api_client.post::<EventResponseWrapper>("events/byids", &request_body){
      Ok(events_response) => {
        let events = events_response.data.items;
        Ok(events)
      },
      Err(e) => Err(e)
    }
  }

  pub fn update(&self, events : &[Event]) -> Result<Vec<Event>> {
    let patch_events : Vec<PatchEvent> = events.iter().map(| e | PatchEvent::from(e)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&patch_events).unwrap());
    match self.api_client.post::<EventResponseWrapper>("events/update", &request_body){
      Ok(events_response) => {
        let events = events_response.data.items;
        Ok(events)
      },
      Err(e) => Err(e)
    }
  }
  
  pub fn search(&self, event_filter : EventFilter, event_search : EventSearch) -> Result<Vec<Event>> {
    let filter : Search = Search::new(event_filter, event_search, None);
    match self.api_client.post::<EventResponseWrapper>("events/search", &serde_json::to_string(&filter).unwrap()) {
      Ok(events_response) => {
        let events = events_response.data.items;
        Ok(events)
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, event_ids : &[u64]) -> Result<()> {
    let id_list : Vec<EventId> = event_ids.iter().map(| e_id | EventId::from(*e_id)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&id_list).unwrap());
    match self.api_client.post::<::serde_json::Value>("events/delete", &request_body){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}

