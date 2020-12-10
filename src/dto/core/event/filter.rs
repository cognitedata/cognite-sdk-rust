use crate::dto::filter_types::EpochTimestampRange;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<EpochTimestampRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<EpochTimestampRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_subtrees: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<EpochTimestampRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<EpochTimestampRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id_prefix: Option<String>,
}

impl EventFilter {
    pub fn new() -> EventFilter {
        EventFilter::default()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedEventCountFilter {
    pub aggregate: String,
    pub filter: EventFilter,
}


impl AggregatedEventCountFilter {
    pub fn new(filter: EventFilter) -> AggregatedEventCountFilter {
        AggregatedEventCountFilter {
            aggregate: "count".to_string(),
            filter
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub filter: EventFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl Filter {
    pub fn new(filter: EventFilter, cursor: Option<String>, limit: Option<u32>) -> Filter {
        Filter {
            filter,
            cursor,
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl EventSearch {
    pub fn new() -> EventSearch {
        EventSearch::default()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub filter: EventFilter,
    pub search: EventSearch,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl Search {
    pub fn new(filter: EventFilter, search: EventSearch, limit: Option<u32>) -> Search {
        Search {
            filter,
            search,
            limit,
        }
    }
}
