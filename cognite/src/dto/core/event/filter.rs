use crate::{
    to_query, to_query_vec, to_query_vec_i64, AsParams, Identity, Partition, Range, SetCursor,
    WithPartition,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct EventQuery {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
    pub min_start_time: Option<i64>,
    pub max_start_time: Option<i64>,
    pub min_end_time: Option<i64>,
    pub max_end_time: Option<i64>,
    pub min_active_time: Option<i64>,
    pub max_active_time: Option<i64>,
    pub asset_ids: Option<Vec<i64>>,
    pub asset_external_ids: Option<Vec<String>>,
    pub asset_subtree_ids: Option<Vec<i64>>,
    pub asset_subtree_external_ids: Option<Vec<String>>,
    pub source: Option<String>,
    pub r#type: Option<String>,
    pub subtype: Option<String>,
    pub min_created_time: Option<i64>,
    pub max_created_time: Option<i64>,
    pub min_last_updated_time: Option<i64>,
    pub max_last_updated_time: Option<i64>,
    pub external_id_prefix: Option<String>,
    pub partition: Option<Partition>,
    pub include_metadata: Option<bool>,
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
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventFilter {
    pub start_time: Option<Range<i64>>,
    pub end_time: Option<Range<i64>>,
    pub active_at_time: Option<Range<i64>>,
    pub metadata: Option<HashMap<String, String>>,
    pub asset_ids: Option<Vec<i64>>,
    pub asset_external_ids: Option<Vec<String>>,
    pub asset_subtree_ids: Option<Vec<Identity>>,
    pub source: Option<String>,
    pub created_time: Option<Range<i64>>,
    pub last_updated_time: Option<Range<i64>>,
    pub external_id_prefix: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub subtype: Option<String>,
    pub data_set_ids: Option<Vec<Identity>>,
}

impl EventFilter {
    pub fn new() -> EventFilter {
        EventFilter::default()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
pub struct EventFilterQuery {
    pub filter: EventFilter,
    pub limit: Option<i32>,
    pub sort: Option<Vec<String>>,
    pub cursor: Option<String>,
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
            limit: self.limit,
            sort: self.sort.clone(),
            cursor: None,
            partition: Some(partition),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedEventsCountFilter {
    pub filter: EventFilter,
}

impl AggregatedEventsCountFilter {
    pub fn new(filter: EventFilter) -> AggregatedEventsCountFilter {
        AggregatedEventsCountFilter { filter }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedEventsListFilter {
    pub filter: EventFilter,
    pub fields: Vec<String>,
    pub aggregate: String,
}

impl AggregatedEventsListFilter {
    pub fn new(
        filter: EventFilter,
        fields: Vec<String>,
        aggregate: String,
    ) -> AggregatedEventsListFilter {
        AggregatedEventsListFilter {
            filter,
            fields,
            aggregate,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventSearch {
    pub description: Option<String>,
}

impl EventSearch {
    pub fn new() -> EventSearch {
        EventSearch::default()
    }
}
