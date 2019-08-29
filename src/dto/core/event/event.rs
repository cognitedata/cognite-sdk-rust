use crate::dto::patch_item::{PatchItem, PatchList};
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
  pub start_time : Option<i64>,
  pub end_time : Option<i64>,
  pub r#type: Option<String>,
  pub subtype: Option<String>,
  pub description : Option<String>,
  pub metadata : Option<HashMap<String, String>>,
  pub asset_ids : Option<Vec<u64>>,
  pub source : Option<String>,
  pub created_time : Option<i64>,
  pub last_updated_time : Option<i64>,
}

impl Event {
  pub fn new(start_time : Option<i64>,
          end_time : Option<i64>,
          r#type: Option<String>,
          subtype: Option<String>,
          external_id: Option<String>,
          description : Option<String>,
          metadata : Option<HashMap<String, String>>,
          asset_ids : Option<Vec<u64>>,
          source : Option<String>) -> Event {
    Event {
      id : 0,
      external_id : external_id,
      start_time : start_time,
      end_time : end_time,
      r#type: r#type,
      subtype: subtype,
      description : description,
      metadata : metadata,
      asset_ids : asset_ids,
      source : source,
      created_time : Some(0),
      last_updated_time : Some(0),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddEvent {
  external_id: Option<String>,
  start_time : Option<i64>,
  end_time : Option<i64>,
  r#type: Option<String>,
  subtype: Option<String>,
  description : Option<String>,
  metadata : Option<HashMap<String, String>>,
  asset_ids : Option<Vec<u64>>,
  source : Option<String>,
}

impl From<&Event> for AddEvent {
  fn from(event : &Event) -> AddEvent {
      AddEvent { 
        external_id : event.external_id.clone(),
        start_time : event.start_time.clone(),
        end_time : event.end_time.clone(),
        r#type : event.r#type.clone(),
        subtype : event.subtype.clone(),
        description : event.description.clone(),
        metadata : event.metadata.clone(),
        asset_ids : event.asset_ids.clone(),
        source : event.source.clone(),
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
  #[serde(skip_serializing_if = "Option::is_none")]
  external_id : Option<PatchItem>,
  #[serde(skip_serializing_if = "Option::is_none")]
  start_time : Option<PatchItem>,
  #[serde(skip_serializing_if = "Option::is_none")]
  end_time : Option<PatchItem>,
  #[serde(skip_serializing_if = "Option::is_none")]
  description : Option<PatchItem>,
  #[serde(skip_serializing_if = "Option::is_none")]
  metadata : Option<PatchItem>,
  #[serde(skip_serializing_if = "Option::is_none")]
  asset_ids : Option<PatchList>,
  #[serde(skip_serializing_if = "Option::is_none")]
  source : Option<PatchItem>,
}

impl From<&Event> for PatchEvent {
  fn from(event : &Event) -> PatchEvent {
    PatchEvent {
      id : event.id,
      update : PatchEventFields {
        external_id : PatchItem::convert_option(&event.external_id),
        start_time : PatchItem::convert_option(&event.start_time),
        end_time : PatchItem::convert_option(&event.end_time),
        description : PatchItem::convert_option(&event.description),
        metadata : PatchItem::convert_option(&event.metadata),
        asset_ids : PatchList::convert(&event.asset_ids),
        source : PatchItem::convert_option(&event.source),
      }
    }
  }
}