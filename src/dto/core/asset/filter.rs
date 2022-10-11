use crate::{
    to_query, AsParams, Identity, LabelsFilter, Partition, Range, SetCursor, WithPartition,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_external_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_ids: Option<Vec<Identity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_subtree_ids: Option<Vec<Identity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_ids: Option<Vec<Identity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<LabelsFilter>,
}

impl AssetFilter {
    pub fn new() -> AssetFilter {
        AssetFilter::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl AssetSearch {
    pub fn new() -> AssetSearch {
        AssetSearch::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilterAssetsRequest {
    pub filter: AssetFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregated_properties: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct AssetQuery {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
    pub include_metadata: Option<bool>,
    pub name: Option<String>,
    pub source: Option<String>,
    pub root: Option<bool>,
    pub min_created_time: Option<i64>,
    pub max_created_time: Option<i64>,
    pub min_last_updated_time: Option<i64>,
    pub max_last_updated_time: Option<i64>,
    pub external_id_prefix: Option<String>,
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
