mod filter;

pub use self::filter::*;

use crate::{
    EqIdentity, Identity, IntegerOrString, IntoPatch, IntoPatchItem, Patch, UpdateList, UpdateMap,
    UpdateSetNull,
};
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
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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

impl IntoPatch<Patch<PatchEvent>> for Event {
    fn patch(self, ignore_nulls: bool) -> Patch<PatchEvent> {
        Patch::<PatchEvent> {
            id: to_idt!(self),
            update: PatchEvent {
                external_id: self.external_id.patch(ignore_nulls),
                data_set_id: self.data_set_id.patch(ignore_nulls),
                start_time: self.start_time.patch(ignore_nulls),
                end_time: self.end_time.patch(ignore_nulls),
                description: self.description.patch(ignore_nulls),
                metadata: self.metadata.patch(ignore_nulls),
                asset_ids: self.asset_ids.patch(ignore_nulls),
                source: self.source.patch(ignore_nulls),
                r#type: self.r#type.patch(ignore_nulls),
                subtype: self.subtype.patch(ignore_nulls),
            },
        }
    }
}

impl IntoPatch<PatchEvent> for AddEvent {
    fn patch(self, ignore_nulls: bool) -> PatchEvent {
        PatchEvent {
            external_id: self.external_id.patch(ignore_nulls),
            data_set_id: self.data_set_id.patch(ignore_nulls),
            start_time: self.start_time.patch(ignore_nulls),
            end_time: self.end_time.patch(ignore_nulls),
            description: self.description.patch(ignore_nulls),
            metadata: self.metadata.patch(ignore_nulls),
            asset_ids: self.asset_ids.patch(ignore_nulls),
            source: self.source.patch(ignore_nulls),
            r#type: self.r#type.patch(ignore_nulls),
            subtype: self.subtype.patch(ignore_nulls),
        }
    }
}

impl From<Event> for Patch<PatchEvent> {
    fn from(value: Event) -> Self {
        IntoPatch::<Patch<PatchEvent>>::patch(value, false)
    }
}
