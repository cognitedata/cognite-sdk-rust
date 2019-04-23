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

  // Search
  #[serde(rename = "description")]
  EventsSearchDescription(String),
  #[serde(rename = "type")]
  EventsSearchType(String),
  #[serde(rename = "subType")]
  EventsSearchSubType(String),
  #[serde(rename = "minStartTime")]
  EventsSearchMinStartTime(u64),
  #[serde(rename = "maxStartTime")]
  EventsSearchMaxStartTime(u64),
  #[serde(rename = "minEndTime")]
  EventsSearchMinEndTime(u64),
  #[serde(rename = "maxEndTime")]
  EventsSearchMaxEndTime(u64),
  #[serde(rename = "minCreatedTime")]
  EventsSearchMinCreatedTime(u64),
  #[serde(rename = "maxCreatedTime")]
  EventsSearchMaxCreatedTime(u64),
  #[serde(rename = "minLastUpdatedTime")]
  EventsSearchMinLastUpdatedTime(u64),
  #[serde(rename = "maxLastUpdatedTime")]
  EventsSearchMaxLastUpdatedTime(u64),
  #[serde(rename = "metadata")]
  EventsSearchMetadata(HashMap<String, String>),
  #[serde(rename = "assetIds")]
  EventsSearchAssetIds(String),
  #[serde(rename = "assetSubtrees")]
  EventsSearchAssetSubtrees(String),
  #[serde(rename = "sort")]
  EventsSearchSort(String),
  #[serde(rename = "dir")]
  EventsSearchDir(String),
  #[serde(rename = "limit")]
  EventsSearchLimit(u32),
  #[serde(rename = "offset")]
  EventsSearchOffset(u32),

  // TIME SERIES

  // Search
  #[serde(rename = "name")]
  TimeSeriesSearchName(String),
  #[serde(rename = "description")]
  TimeSeriesSearchDescription(String),
  #[serde(rename = "query")]
  TimeSeriesSearchQuery(String),
  #[serde(rename = "unit")]
  TimeSeriesSearchUnit(String),
  #[serde(rename = "isString")]
  TimeSeriesSearchIsString(bool),
  #[serde(rename = "isStep")]
  TimeSeriesSearchIsStep(bool),
  #[serde(rename = "metadata")]
  TimeSeriesSearchMetadata(HashMap<String, String>),
  #[serde(rename = "assetIds")]
  TimeSeriesSearchAssetIds(String),
  #[serde(rename = "assetSubtrees")]
  TimeSeriesSearchAssetSubtrees(String),
  #[serde(rename = "minCreatedTime")]
  TimeSeriesSearchMinCreatedTime(u64),
  #[serde(rename = "maxCreatedTime")]
  TimeSeriesSearchMaxCreatedTime(u64),
  #[serde(rename = "minLastUpdatedTime")]
  TimeSeriesSearchMinLastUpdatedTime(u64),
  #[serde(rename = "maxLastUpdatedTime")]
  TimeSeriesSearchMaxLastUpdatedTime(u64),
  #[serde(rename = "sort")]
  TimeSeriesSearchSort(String),
  #[serde(rename = "dir")]
  TimeSeriesSearchDir(String),
  #[serde(rename = "limit")]
  TimeSeriesSearchLimit(u32),
  #[serde(rename = "offset")]
  TimeSeriesSearchOffset(u32),
  #[serde(rename = "boostName")]
  TimeSeriesSearchBoostName(bool),
} 