mod aggregate;
mod filter;

pub use self::aggregate::*;
pub use self::filter::*;

use crate::IdentityOrInstance;
use crate::UpsertOptions;
use crate::{
    EqIdentity, Identity, IntoPatch, IntoPatchItem, Patch, UpdateList, UpdateMap, UpdateSetNull,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// A CDF event.
pub struct Event {
    /// Event internal ID.
    pub id: i64,
    /// Event external ID. Must be unique accross all events in the project.
    pub external_id: Option<String>,
    /// The ID of the dataset this event belongs to.
    pub data_set_id: Option<i64>,
    /// Start time in milliseconds since epoch.
    pub start_time: Option<i64>,
    /// End time in milliseconds since epoch.
    pub end_time: Option<i64>,
    /// Type of the event.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Subtype of the event.
    pub subtype: Option<String>,
    /// Textual description of the event.
    pub description: Option<String>,
    /// Custom, application specific metadata. String key -> String value.
    /// Limits: Maximum length of key is 128 bytes, value 128000 bytes,
    /// up to 256 key-value pairs, of total size at most 200000.
    pub metadata: Option<HashMap<String, String>>,
    /// IDs of assets this event belongs to.
    pub asset_ids: Option<Vec<i64>>,
    /// The source of this event.
    pub source: Option<String>,
    /// Time this event was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this event was last updated, in milliseconds since epoch.
    pub last_updated_time: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Create a new event
pub struct AddEvent {
    /// Event external ID. Must be unique accross all events in the project.
    pub external_id: Option<String>,
    /// The ID of the dataset this event belongs to.
    pub data_set_id: Option<i64>,
    /// Start time in milliseconds since epoch.
    pub start_time: Option<i64>,
    /// End time in milliseconds since epoch.
    pub end_time: Option<i64>,
    /// Type of the event.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Subtype of the event.
    pub subtype: Option<String>,
    /// Textual description of the event.
    pub description: Option<String>,
    /// Custom, application specific metadata. String key -> String value.
    /// Limits: Maximum length of key is 128 bytes, value 128000 bytes,
    /// up to 256 key-value pairs, of total size at most 200000.
    pub metadata: Option<HashMap<String, String>>,
    /// IDs of assets this event belongs to.
    pub asset_ids: Option<Vec<i64>>,
    /// The source of this event.
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
    fn eq(&self, id: &IdentityOrInstance) -> bool {
        match id {
            IdentityOrInstance::Identity(Identity::ExternalId { external_id }) => {
                self.external_id.as_ref() == Some(external_id)
            }
            _ => false,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Update an event.
pub struct PatchEvent {
    /// Event external ID. Must be unique accross all events in the project.
    pub external_id: Option<UpdateSetNull<String>>,
    /// The ID of the dataset this event belongs to.
    pub data_set_id: Option<UpdateSetNull<i64>>,
    /// Start time in milliseconds since epoch.
    pub start_time: Option<UpdateSetNull<i64>>,
    /// End time in milliseconds since epoch.
    pub end_time: Option<UpdateSetNull<i64>>,
    /// Textual description of the event.
    pub description: Option<UpdateSetNull<String>>,
    /// Custom, application specific metadata. String key -> String value.
    /// Limits: Maximum length of key is 128 bytes, value 128000 bytes,
    /// up to 256 key-value pairs, of total size at most 200000.
    pub metadata: Option<UpdateMap<String, String>>,
    /// IDs of assets this event belongs to.
    pub asset_ids: Option<UpdateList<i64, i64>>,
    /// The source of this event.
    pub source: Option<UpdateSetNull<String>>,
    /// Type of the event.
    pub r#type: Option<UpdateSetNull<String>>,
    /// Subtype of the event.
    pub subtype: Option<UpdateSetNull<String>>,
}

impl IntoPatch<Patch<PatchEvent>> for Event {
    fn patch(self, options: &UpsertOptions) -> Patch<PatchEvent> {
        Patch::<PatchEvent> {
            id: to_idt!(self),
            update: PatchEvent {
                external_id: self.external_id.patch(options),
                data_set_id: self.data_set_id.patch(options),
                start_time: self.start_time.patch(options),
                end_time: self.end_time.patch(options),
                description: self.description.patch(options),
                metadata: self.metadata.patch(options),
                asset_ids: self.asset_ids.patch(options),
                source: self.source.patch(options),
                r#type: self.r#type.patch(options),
                subtype: self.subtype.patch(options),
            },
        }
    }
}

impl IntoPatch<PatchEvent> for AddEvent {
    fn patch(self, options: &UpsertOptions) -> PatchEvent {
        PatchEvent {
            external_id: self.external_id.patch(options),
            data_set_id: self.data_set_id.patch(options),
            start_time: self.start_time.patch(options),
            end_time: self.end_time.patch(options),
            description: self.description.patch(options),
            metadata: self.metadata.patch(options),
            asset_ids: self.asset_ids.patch(options),
            source: self.source.patch(options),
            r#type: self.r#type.patch(options),
            subtype: self.subtype.patch(options),
        }
    }
}

impl From<Event> for Patch<PatchEvent> {
    fn from(value: Event) -> Self {
        IntoPatch::<Patch<PatchEvent>>::patch(value, &Default::default())
    }
}
