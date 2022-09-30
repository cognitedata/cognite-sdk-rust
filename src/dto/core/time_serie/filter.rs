use crate::{to_query, Identity, Partition, SetCursor, WithPartition};
use crate::{AsParams, Range};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerieFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_string: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_step: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_external_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_asset_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_subtree_ids: Option<Vec<Identity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_ids: Option<Vec<Identity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<Range<i64>>,
}

impl TimeSerieFilter {
    pub fn new() -> TimeSerieFilter {
        TimeSerieFilter::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerieSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl TimeSerieSearch {
    pub fn new() -> TimeSerieSearch {
        TimeSerieSearch::default()
    }
}

#[derive(Debug, Default)]
pub struct TimeSerieQuery {
    pub limit: Option<i32>,
    pub include_metadata: Option<bool>,
    pub cursor: Option<String>,
    pub partition: Option<Partition>,
    pub external_id_prefix: Option<String>,
}

impl AsParams for TimeSerieQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("limit", &self.limit, &mut params);
        to_query("includeMetadata", &self.include_metadata, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("externalIdPrefix", &self.external_id_prefix, &mut params);
        to_query("partition", &self.partition, &mut params);
        params
    }
}

impl SetCursor for TimeSerieQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl WithPartition for TimeSerieQuery {
    fn with_partition(&self, partition: crate::Partition) -> Self {
        Self {
            limit: self.limit,
            include_metadata: self.include_metadata,
            cursor: None,
            partition: Some(partition),
            external_id_prefix: self.external_id_prefix.clone(),
        }
    }
}
