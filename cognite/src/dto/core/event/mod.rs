mod filter;

pub use self::filter::*;

use crate::{EqIdentity, Identity, IntegerOrString, Patch, UpdateList, UpdateMap, UpdateSetNull};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventListResponse {
    pub items: Vec<Event>,
    previous_cursor: Option<String>,
    next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedCount {
    pub count: i64,
    pub value: IntegerOrString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedEventFilterResponse {
    pub items: Vec<AggregatedCount>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedEventCountResponse {
    pub items: Vec<EventCount>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventCount {
    pub count: i64,
}

impl EventCount {
    pub fn new(count: i64) -> EventCount {
        EventCount { count }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: i64,
    pub external_id: Option<String>,
    pub data_set_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub r#type: Option<String>,
    pub subtype: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub asset_ids: Option<Vec<i64>>,
    pub source: Option<String>,
    pub created_time: Option<i64>,
    pub last_updated_time: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddEvent {
    pub external_id: Option<String>,
    pub data_set_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub r#type: Option<String>,
    pub subtype: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub asset_ids: Option<Vec<i64>>,
    pub source: Option<String>,
}

impl From<Event> for AddEvent {
    fn from(event: Event) -> AddEvent {
        AddEvent {
            external_id: event.external_id,
            data_set_id: event.data_set_id,
            start_time: event.start_time,
            end_time: event.end_time,
            r#type: event.r#type,
            subtype: event.subtype,
            description: event.description,
            metadata: event.metadata,
            asset_ids: event.asset_ids,
            source: event.source,
        }
    }
}

impl EqIdentity for AddEvent {
    fn eq(&self, id: &Identity) -> bool {
        match id {
            Identity::Id { id: _ } => false,
            Identity::ExternalId { external_id } => self.external_id.as_ref() == Some(external_id),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchEvent {
    pub external_id: Option<UpdateSetNull<String>>,
    pub data_set_id: Option<UpdateSetNull<i64>>,
    pub start_time: Option<UpdateSetNull<i64>>,
    pub end_time: Option<UpdateSetNull<i64>>,
    pub description: Option<UpdateSetNull<String>>,
    pub metadata: Option<UpdateMap<String, String>>,
    pub asset_ids: Option<UpdateList<i64, i64>>,
    pub source: Option<UpdateSetNull<String>>,
    pub r#type: Option<UpdateSetNull<String>>,
    pub subtype: Option<UpdateSetNull<String>>,
}

impl From<Event> for Patch<PatchEvent> {
    fn from(event: Event) -> Patch<PatchEvent> {
        Patch::<PatchEvent> {
            id: to_idt!(event),
            update: PatchEvent {
                external_id: Some(event.external_id.into()),
                data_set_id: Some(event.data_set_id.into()),
                start_time: Some(event.start_time.into()),
                end_time: Some(event.end_time.into()),
                description: Some(event.description.into()),
                metadata: Some(event.metadata.into()),
                asset_ids: Some(event.asset_ids.into()),
                source: Some(event.source.into()),
                r#type: Some(event.r#type.into()),
                subtype: Some(event.subtype.into()),
            },
        }
    }
}

impl From<AddEvent> for PatchEvent {
    fn from(event: AddEvent) -> Self {
        PatchEvent {
            external_id: Some(event.external_id.into()),
            data_set_id: Some(event.data_set_id.into()),
            start_time: Some(event.start_time.into()),
            end_time: Some(event.end_time.into()),
            description: Some(event.description.into()),
            metadata: Some(event.metadata.into()),
            asset_ids: Some(event.asset_ids.into()),
            source: Some(event.source.into()),
            r#type: Some(event.r#type.into()),
            subtype: Some(event.subtype.into()),
        }
    }
}
