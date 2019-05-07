use crate::dto::filter_types::{EpochTimestampRange};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventFilter {
  #[serde(skip_serializing_if = "Option::is_none")]
  start_time : Option<EpochTimestampRange>,
  #[serde(skip_serializing_if = "Option::is_none")]
  end_time: Option<EpochTimestampRange>,
  #[serde(skip_serializing_if = "Option::is_none")]
  metadata: Option<HashMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  asset_ids: Option<Vec<u64>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  asset_subtrees: Option<Vec<u64>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  source: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  created_time: Option<EpochTimestampRange>,
  #[serde(skip_serializing_if = "Option::is_none")]
  last_updated_time: Option<EpochTimestampRange>,
  #[serde(skip_serializing_if = "Option::is_none")]
  external_id_prefix: Option<String>
}

impl EventFilter {
  pub fn new() -> EventFilter {
    EventFilter {
      start_time : None,
      end_time: None,
      metadata: None,
      asset_ids: None,
      asset_subtrees: None,
      source: None,
      created_time: None,
      last_updated_time: None,
      external_id_prefix: None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
  filter : EventFilter,
  #[serde(skip_serializing_if = "Option::is_none")]
  cursor : Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  limit : Option<u32>,
}

impl Filter {
  pub fn new(filter : EventFilter, cursor : Option<String>, limit : Option<u32>) -> Filter {
    Filter {
      filter : filter, 
      cursor : cursor,
      limit : limit,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventSearch {
  #[serde(skip_serializing_if = "Option::is_none")]
  description : Option<String>,
}

impl EventSearch {
  pub fn new() -> EventSearch {
    EventSearch {
      description : None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Search {
  filter : EventFilter,
  search : EventSearch,
  #[serde(skip_serializing_if = "Option::is_none")]
  limit : Option<u32>,
}

impl Search {
  pub fn new(filter : EventFilter, search : EventSearch, limit : Option<u32>) -> Search {
    Search {
      filter : filter, 
      search : search,
      limit : limit,
    }
  }
}