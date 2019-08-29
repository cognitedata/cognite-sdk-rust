use crate::dto::filter_types::{EpochTimestampRange, IntegerRange};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_ids: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<EpochTimestampRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<EpochTimestampRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_subtrees: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<IntegerRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id_prefix: Option<String>,
}

impl AssetFilter {
    pub fn new() -> AssetFilter {
        AssetFilter {
            name: None,
            parent_ids: None,
            metadata: None,
            source: None,
            created_time: None,
            last_updated_time: None,
            asset_subtrees: None,
            depth: None,
            external_id_prefix: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AssetSearch {
    pub fn new() -> AssetSearch {
        AssetSearch {
            name: None,
            description: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub filter: AssetFilter,
    pub search: AssetSearch,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl Search {
    pub fn new(filter: AssetFilter, search: AssetSearch, limit: Option<u32>) -> Search {
        Search {
            filter,
            search,
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub filter: AssetFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl Filter {
    pub fn new(filter: AssetFilter, cursor: Option<String>, limit: Option<u32>) -> Filter {
        Filter {
            filter,
            cursor,
            limit,
        }
    }
}
