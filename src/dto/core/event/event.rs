use crate::dto::patch_item::PatchItem;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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
  pub external_id: Option<String>,
  pub start_time : Option<u128>,
  pub end_time : Option<u128>,
  pub description : Option<String>,
  pub metadata : Option<HashMap<String, String>>,
  pub asset_ids : Option<Vec<u64>>,
  pub source : String,
  pub created_time : u128,
  pub last_updated_time : u128,
}

impl Event {
  pub fn new(start_time : Option<u128>,
          end_time : Option<u128>,
          external_id: Option<String>,
          description : Option<String>,
          metadata : Option<HashMap<String, String>>,
          asset_ids : Option<Vec<u64>>,
          source : String) -> Event {
    Event {
      id : 0,
      external_id : external_id,
      start_time : start_time,
      end_time : end_time,
      description : description,
      metadata : metadata,
      asset_ids : asset_ids,
      source : source,
      created_time : 0,
      last_updated_time : 0,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventId {
  id : u64
}

impl From<&Event> for EventId {
  fn from(event : &Event) -> EventId {
    EventId {
      id : event.id
    }
  }
}

impl From<u64> for EventId {
  fn from(event_id : u64) -> EventId {
    EventId {
      id : event_id
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchEvent {
  id : u64,
  update : PatchEventFields
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PatchEventFields {
  external_id : PatchItem,
  start_time : PatchItem,
  end_time : PatchItem,
  description : PatchItem,
  //metadata : PatchItem,
  asset_ids : PatchItem,
  source : PatchItem,
}

impl From<&Event> for PatchEvent {
  fn from(event : &Event) -> PatchEvent {
    PatchEvent {
      id : event.id,
      update : PatchEventFields {
        external_id : PatchItem::from(&event.external_id),
        start_time : PatchItem::from(&event.start_time),
        end_time : PatchItem::from(&event.end_time),
        description : PatchItem::from(&event.description),
        //metadata : PatchItem::from(&event.metadata),
        asset_ids : PatchItem::from(&event.asset_ids),
        source : PatchItem::from(&event.source),
      }
    }
  }
}