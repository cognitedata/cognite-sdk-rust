use crate::dto::filter_types::{EpochTimestampRange, IntegerRange};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetFilter {
  #[serde(skip_serializing_if = "Option::is_none")]
  name : Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  parent_ids: Option<Vec<u32>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  metadata: Option<HashMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  source: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  created_time: Option<EpochTimestampRange>,
  #[serde(skip_serializing_if = "Option::is_none")]
  last_updated_time: Option<EpochTimestampRange>,
  #[serde(skip_serializing_if = "Option::is_none")]
  asset_subtrees: Option<Vec<u64>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  depth: Option<IntegerRange>,
  #[serde(skip_serializing_if = "Option::is_none")]
  external_id_prefix: Option<String>
}

impl AssetFilter {
  pub fn new() -> AssetFilter {
    AssetFilter {
      name : None,
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
  name : Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  description : Option<String>,
}

impl AssetSearch {
  pub fn new() -> AssetSearch {
    AssetSearch {
      name : None,
      description : None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Search {
  filter : AssetFilter,
  search : AssetSearch,
  #[serde(skip_serializing_if = "Option::is_none")]
  limit : Option<u32>,
}

impl Search {
  pub fn new(filter : AssetFilter, search : AssetSearch, limit : Option<u32>) -> Search {
    Search {
      filter : filter, 
      search : search,
      limit : limit,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
  filter : AssetFilter,
  #[serde(skip_serializing_if = "Option::is_none")]
  cursor : Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  limit : Option<u32>,
}

impl Filter {
  pub fn new(filter : AssetFilter, cursor : Option<String>, limit : Option<u32>) -> Filter {
    Filter {
      filter : filter, 
      cursor : cursor,
      limit : limit,
    }
  }
}