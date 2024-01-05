use crate::{
    dto::core::CoreSortItem, to_query, to_query_vec, to_query_vec_i64, AdvancedFilter, AsParams,
    Identity, Partition, Range, SetCursor, WithPartition,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
/// Query for listing events.
pub struct EventQuery {
    /// Maximum number of events to return. The default is 100, and the maximum is 1000.
    pub limit: Option<i32>,
    /// Optional cursor for pagination.
    pub cursor: Option<String>,
    /// Minimum value of `start_time` in milliseconds since epoch.
    pub min_start_time: Option<i64>,
    /// Maximum value of `start_time` in milliseconds since epoch.
    pub max_start_time: Option<i64>,
    /// Minimum value of `end_time` in milliseconds since epoch.
    pub min_end_time: Option<i64>,
    /// Maximum value of `end_time` in milliseconds since epoch.
    pub max_end_time: Option<i64>,
    /// Event is considered active from its startTime to endTime inclusive.
    /// If startTime is null, event is never active. If endTime is null,
    /// event is active from startTime onwards. activeAtTime filter will
    /// match all events that are active at some point from min to max,
    /// from min, or to max, depending on which of min and max parameters are specified.
    /// In milliseconds since epoch.
    pub min_active_time: Option<i64>,
    /// Event is considered active from its startTime to endTime inclusive.
    /// If startTime is null, event is never active. If endTime is null,
    /// event is active from startTime onwards. activeAtTime filter will
    /// match all events that are active at some point from min to max,
    /// from min, or to max, depending on which of min and max parameters are specified.
    /// In milliseconds since epoch.
    pub max_active_time: Option<i64>,
    /// Include events that relate to one of these assets.
    pub asset_ids: Option<Vec<i64>>,
    /// Include events that relate to one of these assets.
    pub asset_external_ids: Option<Vec<String>>,
    /// Include events that relate to assets in the subtree of one of these assets.
    pub asset_subtree_ids: Option<Vec<i64>>,
    /// Include events that relate to assets in the subtree of one of these assets.
    pub asset_subtree_external_ids: Option<Vec<String>>,
    /// Event source.
    pub source: Option<String>,
    /// Event type
    pub r#type: Option<String>,
    /// Event sub-type
    pub subtype: Option<String>,
    /// Minimum value of `created_time` in milliseconds since epoch.
    pub min_created_time: Option<i64>,
    /// Maximum value of `created_time` in milliseconds since epoch.
    pub max_created_time: Option<i64>,
    /// Minimum value of `last_updated_time` in milliseconds since epoch.
    pub min_last_updated_time: Option<i64>,
    /// Maximum value of `last_updated_time` in milliseconds since epoch.
    pub max_last_updated_time: Option<i64>,
    /// Filter by this (case-sensitive) prefix for the external ID.
    pub external_id_prefix: Option<String>,
    /// Split the data set into partitions.
    pub partition: Option<Partition>,
    /// Whether metadata should be returned or not.
    pub include_metadata: Option<bool>,
    /// Sort by an array of selected fields. Syntax is `"<fieldname>:asc|desc"`.
    /// Default sort order is `asc`, with short syntax `"<fieldname>"`.
    ///
    /// Partitions are done independently from sorting, there is no guarantee on sort order between
    /// elements from different partitions.
    pub sort: Option<Vec<String>>,
}

impl AsParams for EventQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("limit", &self.limit, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("minStartTime", &self.min_start_time, &mut params);
        to_query("maxStartTime", &self.max_start_time, &mut params);
        to_query("minEndTime", &self.min_end_time, &mut params);
        to_query("maxEndTime", &self.max_end_time, &mut params);
        to_query("minActiveTime", &self.min_active_time, &mut params);
        to_query("maxActiveTime", &self.max_active_time, &mut params);
        to_query_vec_i64("assetIds", &self.asset_ids, &mut params);
        to_query_vec("assetExternalIds", &self.asset_external_ids, &mut params);
        to_query_vec_i64("assetSubtreeIds", &self.asset_subtree_ids, &mut params);
        to_query_vec(
            "assetSubtreeExternalIds",
            &self.asset_subtree_external_ids,
            &mut params,
        );
        to_query("source", &self.source, &mut params);
        to_query("type", &self.r#type, &mut params);
        to_query("subtype", &self.subtype, &mut params);
        to_query("minCreatedTime", &self.min_created_time, &mut params);
        to_query("maxCreatedTime", &self.max_created_time, &mut params);
        to_query(
            "maxLastUpdatedTime",
            &self.min_last_updated_time,
            &mut params,
        );
        to_query(
            "maxLastUpdatedTime",
            &self.max_last_updated_time,
            &mut params,
        );
        to_query("externalIdPrefix", &self.external_id_prefix, &mut params);
        to_query("partition", &self.partition, &mut params);
        to_query("includeMetadata", &self.include_metadata, &mut params);
        to_query_vec("sort", &self.sort, &mut params);

        params
    }
}

impl SetCursor for EventQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl WithPartition for EventQuery {
    fn with_partition(&self, partition: Partition) -> Self {
        Self {
            limit: self.limit,
            cursor: None,
            min_start_time: self.min_start_time,
            max_start_time: self.max_start_time,
            min_end_time: self.min_end_time,
            max_end_time: self.max_end_time,
            min_active_time: self.min_active_time,
            max_active_time: self.max_active_time,
            asset_ids: self.asset_ids.clone(),
            asset_external_ids: self.asset_external_ids.clone(),
            asset_subtree_ids: self.asset_subtree_ids.clone(),
            asset_subtree_external_ids: self.asset_subtree_external_ids.clone(),
            source: self.source.clone(),
            r#type: self.r#type.clone(),
            subtype: self.subtype.clone(),
            min_created_time: self.min_created_time,
            max_created_time: self.max_created_time,
            min_last_updated_time: self.min_last_updated_time,
            max_last_updated_time: self.max_last_updated_time,
            external_id_prefix: self.external_id_prefix.clone(),
            partition: Some(partition),
            include_metadata: self.include_metadata,
            sort: self.sort.clone(),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum RangeOrIsNull {
    Range(Range<i64>),
    IsNull { is_null: bool },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Simple filter on events.
pub struct EventFilter {
    /// Range of timestamps for `start_time`.
    pub start_time: Option<Range<i64>>,
    /// Range of timestamp for `end_time`, or `isNull` filter condition.
    pub end_time: Option<RangeOrIsNull>,
    /// Event is considered active from its startTime to endTime inclusive.
    /// If startTime is null, event is never active. If endTime is null,
    /// event is active from startTime onwards. activeAtTime filter will match
    /// all events that are active at some point from min to max, from min, or to max,
    /// depending on which of min and max parameters are specified.
    pub active_at_time: Option<Range<i64>>,
    /// Filter on event metadata.
    pub metadata: Option<HashMap<String, String>>,
    /// Include events that relate to one of these assets.
    pub asset_ids: Option<Vec<i64>>,
    /// Include events that relate to one of these assets.
    pub asset_external_ids: Option<Vec<String>>,
    /// Include events that relate to assets in the subtree of one of these assets.
    pub asset_subtree_ids: Option<Vec<Identity>>,
    /// Event source.
    pub source: Option<String>,
    /// Range of timestamps for `created_time`.
    pub created_time: Option<Range<i64>>,
    /// Range of timestamps for `last_updated_time`.
    pub last_updated_time: Option<Range<i64>>,
    /// Filter by this (case-sensitive) prefix for the external ID.
    pub external_id_prefix: Option<String>,
    #[serde(rename = "type")]
    /// Event type
    pub r#type: Option<String>,
    /// Event sub-type
    pub subtype: Option<String>,
    /// Include events which are tied to one of these data sets.
    pub data_set_ids: Option<Vec<Identity>>,
}

impl EventFilter {
    pub fn new() -> EventFilter {
        EventFilter::default()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
/// Request for filtering events.
pub struct EventFilterQuery {
    /// Simple event filter.
    pub filter: Option<EventFilter>,
    /// Advanced filter.
    pub advanced_filter: Option<AdvancedFilter>,
    /// Maximum number of events to return. The default is 100, and the maximum is 1000.
    pub limit: Option<i32>,
    /// Sort result by list of properties. The order is significant.
    pub sort: Option<Vec<CoreSortItem>>,
    /// Optional cursor for pagination.
    pub cursor: Option<String>,
    /// Split the data set into partitions.
    pub partition: Option<Partition>,
}

impl SetCursor for EventFilterQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl WithPartition for EventFilterQuery {
    fn with_partition(&self, partition: Partition) -> Self {
        Self {
            filter: self.filter.clone(),
            advanced_filter: self.advanced_filter.clone(),
            limit: self.limit,
            sort: self.sort.clone(),
            cursor: None,
            partition: Some(partition),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Fuzzy search on events.
pub struct EventSearch {
    /// Text to search in event description.
    pub description: Option<String>,
}

impl EventSearch {
    /// Create an empty event search.
    pub fn new() -> EventSearch {
        EventSearch::default()
    }
}
