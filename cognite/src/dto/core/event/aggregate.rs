use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{AdvancedFilter, AggregateFilter};

use super::EventFilter;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Descriptor for event properties to compute aggregates on.
pub struct AggregateProperty {
    /// An array of strings specifying a nested property.
    property: Vec<String>,
    /// Filter on which property values to include.
    filter: Option<AggregateFilter>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", untagged)]
/// Variants of the `count` aggregate on events.
pub enum EventAggregateCount {
    /// Count the number of events with a given property (non-null),
    /// matching the filters.
    PropertyCount {
        /// Advanced filter on events.
        advanced_filter: Option<AdvancedFilter>,
        /// Simple filter on events.
        filter: Option<EventFilter>,
        /// Properties to apply the aggration on. Currently limited to one property per request.
        properties: Vec<AggregateProperty>,
    },
    /// Count the number of events matching filters.
    EventCount {
        /// Advanced filter on events.
        advanced_filter: Option<AdvancedFilter>,
        /// Simple filter on events.
        filter: Option<EventFilter>,
    },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AggregateWithProperty {
    /// Filter on aggregate property values.
    aggregate_filter: Option<AggregateFilter>,
    /// Advanced filter on events.
    advanced_filter: Option<AdvancedFilter>,
    /// Simple filter on events.
    filter: Option<EventFilter>,
    /// Properties to apply the aggration on. Currently limited to one property per request.
    properties: Vec<AggregateProperty>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AggregateWithPath {
    /// Filter on aggregate property values.
    aggregate_filter: Option<AggregateFilter>,
    /// Advanced filter on events.
    advanced_filter: Option<AdvancedFilter>,
    /// Simple filter on events.
    filter: Option<EventFilter>,
    /// Scope in each document to aggregate properties. Currently the only allowed value is
    /// `["metadata"]`, meaning aggregates are computed on metadata properties.
    path: Vec<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Request for aggregates on events.
pub enum EventAggregateRequest {
    /// Count the number of events matching filters.
    Count(EventAggregateCount),
    /// Compute the approximate number of unique values for the specified property.
    CardinalityValues(AggregateWithProperty),
    /// Compute the approximate number of unique metadata properties.
    CardinalityProperties(AggregateWithPath),
    /// Get up to 1000 unique values for the specified property ordered by frequency.
    /// Note: when aggregating on metadata, a value may occur multiple times in one asset
    /// for different metadata keys. Each occurence is counted.
    UniqueValues(AggregateProperty),
    /// Get unique metadata keys in a given asset. Ordered by frequency.
    UniqueProperties(AggregateWithPath),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// Describes a property in an event.
pub struct AggregatedProperty {
    /// Path to the property.
    property: Vec<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", untagged)]
/// Response for an event aggregation request. The type of result depends
/// on the requested aggregate.
pub enum EventAggregateResponse {
    /// Aggregate with a list of string values
    Strings {
        /// Number of items in this bucket.
        count: i64,
        /// Array of unique values in the property.
        values: Vec<String>,
    },
    /// Aggregate with a list of integer values.
    Integers {
        /// Number of items in this bucket.
        count: i64,
        /// Array of unique values in the property.
        values: Vec<i64>,
    },
    /// A bucket representing the result of the `UniqueProperties` aggregate.
    Properties {
        /// Number of items in this bucket.
        count: i64,
        /// An array of unique properties.
        values: Vec<AggregatedProperty>,
    },
    /// Aggregate returned when only a simple count is requested.
    Count {
        /// Number of items in this aggregation group.
        count: i64,
    },
}
