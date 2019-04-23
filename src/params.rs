use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Params {
  // ASSET

  // Search
  #[serde(rename = "name")]
  AssetSearchName(String),
  #[serde(rename = "description")]
  AssetSearchDescription(String),
  #[serde(rename = "query")]
  AssetSearchQuery(String),
  #[serde(rename = "metadata")]
  AssetSearchMetadata(HashMap<String, String>),
  #[serde(rename = "assetSubtrees")]
  AssetSearchAssetSubtrees(String),
  #[serde(rename = "minCreatedTime")]
  AssetSearchMinCreatedTime(u64),
  #[serde(rename = "maxCreatedTime")]
  AssetSearchMaxCreatedTime(u64),
  #[serde(rename = "minLastUpdatedTime")]
  AssetSearchMinLastUpdatedTime(u64),
  #[serde(rename = "maxLastUpdatedTime")]
  AssetSearchMaxLastUpdatedTime(u64),
  #[serde(rename = "sort")]
  AssetSearchSort(String),
  #[serde(rename = "dir")]
  AssetSearchDir(String),
  #[serde(rename = "limit")]
  AssetSearchLimit(u32),
  #[serde(rename = "offset")]
  AssetSearchOffset(u32),
  #[serde(rename = "boostName")]
  AssetSearchBoostName(bool),

  // ListAll
  #[serde(rename = "name")]
  AssetListAllName(String),
  #[serde(rename = "fuzziness")]
  AssetListAllFuzziness(u32),
  #[serde(rename = "path")]
  AssetListAllPath(String),
  #[serde(rename = "depth")]
  AssetListAllDepth(String),
  #[serde(rename = "metadata")]
  AssetListAllMetadata(HashMap<String,String>),
  #[serde(rename = "description")]
  AssetListAllDescription(String),
  #[serde(rename = "source")]
  AssetListAllSource(String),
  #[serde(rename = "cursor")]
  AssetListAllCursor(String),
  #[serde(rename = "limit")]
  AssetListAllLimit(u32),
} 