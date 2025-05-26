pub(crate) mod aggregates;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{time_series::TimestampOrRelative, AdvancedFilter, PropertyIdentifier};

use super::{
    common::{SortDirection, TaggedContainerReference},
    instances::PropertiesObject,
};

/// Matches records with the last updated time within the provided range.
///
/// The range must include at least a left (gt or gte) bound.
/// It is not allowed to specify two upper or lower bounds, e.g. gte and gt,
/// in the same filter.
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastUpdatedTimeFilter {
    /// Greater than or equal to
    pub gte: Option<TimestampOrRelative>,
    /// Greater than
    pub gt: Option<TimestampOrRelative>,
    /// Less than or equal to
    pub lte: Option<TimestampOrRelative>,
    /// Less than
    pub lt: Option<TimestampOrRelative>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Data to be written to a record.
/// The `TProperties` type parameter is used to specify the property object.
/// A generic version could be `HashMap<String, RawValue>`.
pub struct RecordData<TProperties> {
    /// The container of the property.
    pub source: TaggedContainerReference,
    /// The properties to be written.
    pub properties: TProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Create/update of a record.
pub struct RecordWrite<TProperties> {
    /// The space of the record.
    pub space: String,
    /// The external ID of the record.
    pub external_id: String,
    /// The properties to be written.
    pub sources: Vec<RecordData<TProperties>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A record retrieved from CDF.
/// The `TProperties` type parameter is used to specify the property object.
/// A generic version could be `HashMap<String, RawValue>`.
pub struct Record<TProperties> {
    /// The space of the record.
    pub space: String,
    /// The external ID of the record.
    pub external_id: String,
    /// The properties of the record, as a dictionary from
    /// space to container to property object.
    pub properties: PropertiesObject<TProperties>,
    /// Time this record was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this record was last modified, in milliseconds since epoch.
    pub last_updated_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Which properties to retrieve from a container.
pub struct PropertiesPerContainer {
    /// The container to retrieve properties from.
    pub source: TaggedContainerReference,
    /// The properties to retrieve.
    pub properties: Vec<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Sort on a dynamic property
pub struct RecordsPropertySort {
    /// List of strings representing the property
    pub property: Vec<String>,
    /// Direction to sort.
    pub direction: Option<SortDirection>,
}

impl RecordsPropertySort {
    /// Create a new property sort object.
    ///
    /// # Arguments
    ///
    /// * `property` - Property to sort by
    /// * `direction` - Direction to sort in.
    pub fn new(property: impl PropertyIdentifier, direction: SortDirection) -> Self {
        Self {
            property: property.into_identifier(),
            direction: Some(direction),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Request to retrieve records from CDF.
pub struct RecordsRetrieveRequest {
    /// The time the record was last updated.
    pub last_updated_time: LastUpdatedTimeFilter,
    /// The filter to apply to the records.
    pub filter: Option<AdvancedFilter>,
    /// The properties to retrieve.
    pub sources: Option<Vec<PropertiesPerContainer>>,
    /// Limit the number of records to retrieve, defaults to 10.
    pub limit: Option<u32>,
    /// Optionally sort the records.
    pub sort: Option<Vec<RecordsPropertySort>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An cursor to send to the records API in a sync request.
pub enum RecordCursor {
    /// A cursor received from a previous request.
    Cursor(String),
    /// The value to use to initialize the cursor.
    /// On the form `[duration]-ago`, for example `3m-ago`, `1h-ago`, etc.
    InitializeCursor(String),
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Request to sync records from CDF.
pub struct RecordsSyncRequest {
    /// The properties to retrieve.
    pub sources: Option<Vec<PropertiesPerContainer>>,
    /// An optional filter to apply to the records.
    pub filter: Option<AdvancedFilter>,
    /// Limit the number of records to retrieve, defaults to 10.
    pub limit: Option<u32>,
    /// When to initialize the cursor
    #[serde(flatten)]
    pub cursor: RecordCursor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Part of the records sync response, containing `has_next` and `next_cursor`.
pub struct CursorAndHasNext {
    /// The cursor to use in the next request.
    pub next_cursor: String,
    /// Whether there are more records to retrieve.
    /// If this is `false`, the client should back off and wait a bit before
    /// asking again.
    pub has_next: bool,
}
