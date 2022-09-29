mod filter;

pub use self::filter::*;

use crate::{EqIdentity, Identity, Patch, UpdateList, UpdateMap, UpdateSetNull};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventListResponse {
    pub items: Vec<Event>,
    previous_cursor: Option<String>,
    next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum IntegerOrString {
    Integer(i64),
    String(String),
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Default)]
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

impl From<&Event> for AddEvent {
    fn from(event: &Event) -> AddEvent {
        AddEvent {
            external_id: event.external_id.clone(),
            data_set_id: event.data_set_id,
            start_time: event.start_time,
            end_time: event.end_time,
            r#type: event.r#type.clone(),
            subtype: event.subtype.clone(),
            description: event.description.clone(),
            metadata: event.metadata.clone(),
            asset_ids: event.asset_ids.clone(),
            source: event.source.clone(),
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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<UpdateMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<UpdateList<i64, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<UpdateSetNull<String>>,
}

impl From<&Event> for Patch<PatchEvent> {
    fn from(event: &Event) -> Patch<PatchEvent> {
        Patch::<PatchEvent> {
            id: Identity::Id { id: event.id },
            update: PatchEvent {
                external_id: Some(event.external_id.clone().into()),
                data_set_id: Some(event.data_set_id.into()),
                start_time: Some(event.start_time.into()),
                end_time: Some(event.end_time.into()),
                description: Some(event.description.clone().into()),
                metadata: Some(event.metadata.clone().into()),
                asset_ids: Some(event.asset_ids.clone().into()),
                source: Some(event.source.clone().into()),
                r#type: Some(event.r#type.clone().into()),
                subtype: Some(event.subtype.clone().into()),
            },
        }
    }
}

impl From<&AddEvent> for PatchEvent {
    fn from(event: &AddEvent) -> Self {
        PatchEvent {
            external_id: Some(event.external_id.clone().into()),
            data_set_id: Some(event.data_set_id.into()),
            start_time: Some(event.start_time.into()),
            end_time: Some(event.end_time.into()),
            description: Some(event.description.clone().into()),
            metadata: Some(event.metadata.clone().into()),
            asset_ids: Some(event.asset_ids.clone().into()),
            source: Some(event.source.clone().into()),
            r#type: Some(event.r#type.clone().into()),
            subtype: Some(event.subtype.clone().into()),
        }
    }
}
