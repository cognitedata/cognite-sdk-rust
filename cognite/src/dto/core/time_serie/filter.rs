use crate::{to_query, Identity, Partition, SetCursor, WithPartition};
use crate::{AsParams, Range};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerieFilter {
    pub name: Option<String>,
    pub unit: Option<String>,
    pub is_string: Option<bool>,
    pub is_step: Option<bool>,
    pub metadata: Option<HashMap<String, String>>,
    pub asset_ids: Option<Vec<i64>>,
    pub asset_external_ids: Option<Vec<String>>,
    pub root_asset_ids: Option<Vec<String>>,
    pub asset_subtree_ids: Option<Vec<Identity>>,
    pub data_set_ids: Option<Vec<Identity>>,
    pub external_id_prefix: Option<String>,
    pub created_time: Option<Range<i64>>,
    pub last_updated_time: Option<Range<i64>>,
}

impl TimeSerieFilter {
    pub fn new() -> TimeSerieFilter {
        TimeSerieFilter::default()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerieSearch {
    pub name: Option<String>,
    pub description: Option<String>,
    pub query: Option<String>,
}

impl TimeSerieSearch {
    pub fn new() -> TimeSerieSearch {
        TimeSerieSearch::default()
    }
}

#[derive(Debug, Default, Clone)]
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
