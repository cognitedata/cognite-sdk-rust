use crate::dto::filter_types::{EpochTimestampRange};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerieFilter {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub unit : Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_string: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_step: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub metadata: Option<HashMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub asset_ids: Option<Vec<u64>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub asset_subtrees: Option<Vec<u64>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_time: Option<EpochTimestampRange>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last_updated_time: Option<EpochTimestampRange>,
}

impl TimeSerieFilter {
  pub fn new() -> TimeSerieFilter {
    TimeSerieFilter {
      unit : None,
      is_string: None,
      is_step : None,
      metadata: None,
      asset_ids: None,
      asset_subtrees: None,
      created_time: None,
      last_updated_time: None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerieSearch {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name : Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description : Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub query : Option<String>,
}

impl TimeSerieSearch {
  pub fn new() -> TimeSerieSearch {
    TimeSerieSearch {
      name : None,
      description : None,
      query : None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Search {
  pub filter : TimeSerieFilter,
  pub search : TimeSerieSearch,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit : Option<u32>,
}

impl Search {
  pub fn new(filter : TimeSerieFilter, search : TimeSerieSearch, limit : Option<u32>) -> Search {
    Search {
      filter : filter, 
      search : search,
      limit : limit,
    }
  }
}