use crate::dto::core::CoreSortItem;
use crate::{
    to_query, to_query_vec_i64, AdvancedFilter, Identity, Partition, SetCursor, WithPartition,
};
use crate::{AsParams, Range};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesFilter {
    /// Include time series with this name.
    pub name: Option<String>,
    /// Include time series with this unit.
    pub unit: Option<String>,
    /// Include time series with this unit external ID.
    pub unit_external_id: Option<String>,
    /// Include time series with this unit quantity.
    pub unit_quantity: Option<String>,
    /// Include time series with this value for `is_string`.
    pub is_string: Option<bool>,
    /// Include time series with this value for `is_step`
    pub is_step: Option<bool>,
    /// Filter on time series metadata.
    pub metadata: Option<HashMap<String, String>>,
    /// Include time series that relate to one of these assets.
    pub asset_ids: Option<Vec<i64>>,
    /// Include time series that relate to one of these assets.
    pub asset_external_ids: Option<Vec<String>>,
    /// Include time series that relate to assets in the tree of one of these root assets
    pub root_asset_ids: Option<Vec<String>>,
    /// Include time series that relate to assets in the subtree of one of these assets
    pub asset_subtree_ids: Option<Vec<Identity>>,
    /// Include time series which are tied to one of these data sets.
    pub data_set_ids: Option<Vec<Identity>>,
    /// Filter by this (case-sensitive) prefix for the external ID.
    pub external_id_prefix: Option<String>,
    /// Range of timestamps for `created_time`.
    pub created_time: Option<Range<i64>>,
    /// Range of timestamps for `last_updated_time`.
    pub last_updated_time: Option<Range<i64>>,
}

impl TimeSeriesFilter {
    /// Create an empty time series filter.
    pub fn new() -> TimeSeriesFilter {
        TimeSeriesFilter::default()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Fuzzy search on time series properties.
pub struct TimeSeriesSearch {
    /// Fuzzy search on name.
    pub name: Option<String>,
    /// Fuzzy search on description.
    pub description: Option<String>,
    /// Whitespace-separated terms to search for in time series.
    /// Does a best-effort fuzzy search in relevant fields (currently name and description)
    /// for variations of any search terms and orders results by relevance.
    /// Uses a different search algorithm than the name and description parameters
    /// and will generally give much better results. Matching and ordering aren't
    /// guaranteed to be stable over time, and the fields being searched may be extended.
    pub query: Option<String>,
}

impl TimeSeriesSearch {
    /// Create an empty time series search.
    pub fn new() -> TimeSeriesSearch {
        TimeSeriesSearch::default()
    }
}

#[derive(Debug, Default, Clone)]
/// Query for listing time series.
pub struct TimeSeriesQuery {
    /// Max number of returned time series. Default is 100, maximum is 1000.
    pub limit: Option<i32>,
    /// Whether the metadata field should be returned or not.
    pub include_metadata: Option<bool>,
    /// Cursor for pagination.
    pub cursor: Option<String>,
    /// Split the data set into partitions.
    pub partition: Option<Partition>,
    /// Filter by this (case-sensitive) prefix for the external ID.
    pub external_id_prefix: Option<String>,
    /// Include time series which belong to one of these assets.
    pub asset_ids: Option<Vec<i64>>,
    /// Include time series which belong to an asset in the tree of one of these
    /// root assets.
    pub root_asset_ids: Option<Vec<i64>>,
}

impl AsParams for TimeSeriesQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("limit", &self.limit, &mut params);
        to_query("includeMetadata", &self.include_metadata, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("externalIdPrefix", &self.external_id_prefix, &mut params);
        to_query("partition", &self.partition, &mut params);
        to_query_vec_i64("assetIds", &self.asset_ids, &mut params);
        to_query_vec_i64("rootAssetIds", &self.root_asset_ids, &mut params);
        params
    }
}

impl SetCursor for TimeSeriesQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl WithPartition for TimeSeriesQuery {
    fn with_partition(&self, partition: crate::Partition) -> Self {
        Self {
            limit: self.limit,
            include_metadata: self.include_metadata,
            cursor: None,
            partition: Some(partition),
            external_id_prefix: self.external_id_prefix.clone(),
            asset_ids: self.asset_ids.clone(),
            root_asset_ids: self.root_asset_ids.clone(),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Request for filtering time series.
pub struct TimeSeriesFilterRequest {
    /// Simple time series. filter.
    pub filter: Option<TimeSeriesFilter>,
    /// Advanced filter.
    pub advanced_filter: Option<AdvancedFilter>,
    /// Max number of returned time series. Default is 100, maximum is 1000.
    pub limit: Option<i32>,
    /// Cursor for pagination.
    pub cursor: Option<String>,
    /// Split the result set into partitions.
    pub partition: Option<Partition>,
    /// Sort the returned time series. The order is significant.
    pub sort: Option<Vec<CoreSortItem>>,
}

impl SetCursor for TimeSeriesFilterRequest {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl WithPartition for TimeSeriesFilterRequest {
    fn with_partition(&self, partition: Partition) -> Self {
        let mut copy = self.clone();
        copy.partition = Some(partition);
        copy
    }
}
