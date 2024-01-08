use crate::{
    dto::core::common::CoreSortItem, to_query, AdvancedFilter, AsParams, Identity, LabelsFilter,
    Partition, Range, SetCursor, WithPartition,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use super::AssetAggregatedProperty;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Simple filter on assets.
pub struct AssetFilter {
    /// The name of the asset.
    pub name: Option<String>,
    /// Return only direct descendants of the specified assets.
    pub parent_ids: Option<Vec<i64>>,
    /// Return only direct descendants of the specified assets.
    pub parent_external_ids: Option<Vec<String>>,
    /// DEPRECATED: Use `asset_subtree_ids` instead.
    ///
    /// Only include these root assets and their descendants.
    pub root_ids: Option<Vec<Identity>>,
    /// Only include assets in subtrees rooted at the specified assets (including the roots given).
    /// If the total size of the given subtrees exceeds 100,000 assets, an error will be returned.
    pub asset_subtree_ids: Option<Vec<Identity>>,
    /// List of data set external or internal IDs that included assets will belong to.
    pub data_set_ids: Option<Vec<Identity>>,
    /// Filter on metadata keys and values.
    pub metadata: Option<HashMap<String, String>>,
    /// The source of the asset.
    pub source: Option<String>,
    /// Range for the `created_time` field on included assets.
    pub created_time: Option<Range<i64>>,
    /// Range for the `last_updated_time` field on included assets.
    pub last_updated_time: Option<Range<i64>>,
    /// Filter by this (case-sensitive) prefix for the asset external ID.
    pub external_id_prefix: Option<String>,
    /// Whether the included assets are root assets or not.
    pub root: Option<bool>,
    /// Return only assets matching the given labels filter.
    pub labels: Option<LabelsFilter>,
}

impl AssetFilter {
    /// Create an empty assets filter.
    pub fn new() -> AssetFilter {
        AssetFilter::default()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Fuzzy search on asset properties.
pub struct AssetSearch {
    /// The name of the asset
    pub name: Option<String>,
    /// The description of the asset.
    pub description: Option<String>,
    /// Whitespace-separated terms to search for in assets.
    /// Does a best-effort fuzzy search in relevant fields (currently name and description)
    /// for variations of any of the search terms,
    /// and orders results by relevance. Uses a different search algorithm than the
    /// name and description parameters, and will generally give much better results.
    /// Matching and ordering is not guaranteed to be stable over time, and the fields
    /// being searched may be extended.
    pub query: Option<String>,
}

impl AssetSearch {
    /// Create an empty assets search
    pub fn new() -> AssetSearch {
        AssetSearch::default()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Request for filtering assets.
pub struct FilterAssetsRequest {
    /// Simple filter.
    pub filter: Option<AssetFilter>,
    /// Advanced filter.
    pub advanced_filter: Option<AdvancedFilter>,
    /// Maximum number of results to return.
    pub limit: Option<i32>,
    /// Optional cursor for pagination.
    pub cursor: Option<String>,
    /// Aggregated properties to include.
    pub aggregated_properties: Option<Vec<AssetAggregatedProperty>>,
    /// Optional partition.
    pub partition: Option<Partition>,
    /// Optional list of fields to sort on. The order is significant.
    pub sort: Option<Vec<CoreSortItem>>,
}

impl SetCursor for FilterAssetsRequest {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl WithPartition for FilterAssetsRequest {
    fn with_partition(&self, partition: Partition) -> Self {
        let mut copy = self.clone();
        copy.partition = Some(partition);
        copy
    }
}

#[derive(Debug, Default, Clone)]
/// Query parameters for filtering assets.
pub struct AssetQuery {
    /// Maximum number of results to return.
    pub limit: Option<i32>,
    /// Optional cursor for pagination.
    pub cursor: Option<String>,
    /// Whether the metadata field should be returned or not.
    pub include_metadata: Option<bool>,
    /// The name of the assets to include.
    pub name: Option<String>,
    /// The source of the assets to include.
    pub source: Option<String>,
    /// Whether the filtered assets are root assets or not.
    pub root: Option<bool>,
    /// Min value of `created_time`, in milliseconds since epoch.
    pub min_created_time: Option<i64>,
    /// Max value of `created_time`, in milliseconds since epoch.
    pub max_created_time: Option<i64>,
    /// Min value of `last_updated_time`, in milliseconds since epoch.
    pub min_last_updated_time: Option<i64>,
    /// Max value of `last_updated_time`, in milliseconds since epoch.
    pub max_last_updated_time: Option<i64>,
    /// Filter by this (case-sensitive) prefix for the external ID.
    pub external_id_prefix: Option<String>,
    /// Split the data set into partitions.
    pub partition: Option<Partition>,
}

impl AsParams for AssetQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("limit", &self.limit, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("includeMetadata", &self.include_metadata, &mut params);
        to_query("name", &self.name, &mut params);
        to_query("source", &self.source, &mut params);
        to_query("root", &self.root, &mut params);
        to_query("minCreatedTime", &self.min_created_time, &mut params);
        to_query("maxCreatedTime", &self.max_created_time, &mut params);
        to_query(
            "minLastUpdatedTime",
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
        params
    }
}

impl SetCursor for AssetQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl WithPartition for AssetQuery {
    fn with_partition(&self, partition: Partition) -> Self {
        Self {
            limit: self.limit,
            cursor: self.cursor.clone(),
            include_metadata: self.include_metadata,
            name: self.name.clone(),
            source: self.source.clone(),
            root: self.root,
            min_created_time: self.min_created_time,
            max_created_time: self.max_created_time,
            min_last_updated_time: self.min_last_updated_time,
            max_last_updated_time: self.max_last_updated_time,
            external_id_prefix: self.external_id_prefix.clone(),
            partition: Some(partition),
        }
    }
}
