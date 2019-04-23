use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Params {
  // ASSET

  // Search
  #[serde(rename = "name")]
  AssetsSearchName(String),
  #[serde(rename = "description")]
  AssetsSearchDescription(String),
  #[serde(rename = "query")]
  AssetsSearchQuery(String),
  #[serde(rename = "metadata")]
  AssetsSearchMetadata(HashMap<String, String>),
  #[serde(rename = "assetSubtrees")]
  AssetsSearchAssetSubtrees(String),
  #[serde(rename = "minCreatedTime")]
  AssetsSearchMinCreatedTime(u64),
  #[serde(rename = "maxCreatedTime")]
  AssetsSearchMaxCreatedTime(u64),
  #[serde(rename = "minLastUpdatedTime")]
  AssetsSearchMinLastUpdatedTime(u64),
  #[serde(rename = "maxLastUpdatedTime")]
  AssetsSearchMaxLastUpdatedTime(u64),
  #[serde(rename = "sort")]
  AssetsSearchSort(String),
  #[serde(rename = "dir")]
  AssetsSearchDir(String),
  #[serde(rename = "limit")]
  AssetsSearchLimit(u32),
  #[serde(rename = "offset")]
  AssetsSearchOffset(u32),
  #[serde(rename = "boostName")]
  AssetsSearchBoostName(bool),

  // ListAll
  #[serde(rename = "name")]
  AssetsListAllName(String),
  #[serde(rename = "fuzziness")]
  AssetsListAllFuzziness(u32),
  #[serde(rename = "path")]
  AssetsListAllPath(String),
  #[serde(rename = "depth")]
  AssetsListAllDepth(String),
  #[serde(rename = "metadata")]
  AssetsListAllMetadata(HashMap<String,String>),
  #[serde(rename = "description")]
  AssetsListAllDescription(String),
  #[serde(rename = "source")]
  AssetsListAllSource(String),
  #[serde(rename = "cursor")]
  AssetsListAllCursor(String),
  #[serde(rename = "limit")]
  AssetsListAllLimit(u32),

  // EVENTS 

  // ListAll
  #[serde(rename = "type")]
  EventsListAllType(String),
  #[serde(rename = "subType")]
  EventsListAllSubType(String),
  #[serde(rename = "assetId")]
  EventsListAllAssetId(u64),
  #[serde(rename = "sort")]
  EventsListAllSort(String),
  #[serde(rename = "cursor")]
  EventsListAllCursor(String),
  #[serde(rename = "limit")]
  EventsListAllLimit(u32),
  #[serde(rename = "hasDescription")]
  EventsListAllHasDescription(bool),
  #[serde(rename = "minStartTime")]
  EventsListAllMinStartTime(u64),
  #[serde(rename = "maxStartTime")]
  EventsListAllMaxStartTime(u64),
  #[serde(rename = "source")]
  EventsListAllSource(String),
} 