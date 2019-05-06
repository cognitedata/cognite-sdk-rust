use crate::dto::filter_types::{EpochTimestampRange};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileFilter {
  #[serde(skip_serializing_if = "Option::is_none")]
  metadata: Option<HashMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  asset_ids: Option<Vec<u64>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  source: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  created_time: Option<EpochTimestampRange>,
  #[serde(skip_serializing_if = "Option::is_none")]
  last_updated_time: Option<EpochTimestampRange>,
  #[serde(skip_serializing_if = "Option::is_none")]
  external_id_prefix: Option<String>
}

impl FileFilter {
  pub fn new() -> FileFilter {
    FileFilter {
      metadata: None,
      asset_ids: None,
      source: None,
      created_time: None,
      last_updated_time: None,
      external_id_prefix: None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileSearch {
  #[serde(skip_serializing_if = "Option::is_none")]
  name : Option<String>,
}

impl FileSearch {
  pub fn new() -> FileSearch {
    FileSearch {
      name : None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Search {
  filter : FileFilter,
  search : FileSearch,
  #[serde(skip_serializing_if = "Option::is_none")]
  limit : Option<u32>,
}

impl Search {
  pub fn new(filter : FileFilter, search : FileSearch, limit : Option<u32>) -> Search {
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
  filter : FileFilter,
  #[serde(skip_serializing_if = "Option::is_none")]
  cursor : Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  limit : Option<u32>,
}

impl Filter {
  pub fn new(filter : FileFilter, cursor : Option<String>, limit : Option<u32>) -> Filter {
    Filter {
      filter : filter, 
      cursor : cursor,
      limit : limit,
    }
  }
}