use super::patch_item::PatchItem;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponseWrapper {
  pub data : AssetResponse
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponse {
  pub items : Vec<Asset>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
  pub name : String,
  pub id : u64,
  pub ref_id : Option<String>,
  pub parent_id : Option<u64>,
  pub parent_ref_id : Option<String>,
  pub description : String,
  pub depth: u64,
  pub metadata: Option<HashMap<String, String>>,
  pub source : Option<String>,
  pub source_id : Option<u64>,
  pub created_time : u128,
  pub last_updated_time : u128,
  pub path : Vec<u64>
}

impl Asset {
  pub fn new(name : &str, 
            description : &str, 
            parent_id : Option<u64>,
            metadata : Option<HashMap<String, String>>,
            source : Option<String>,
            source_id : Option<u64>) -> Asset {
    Asset {
      name : String::from(name),
      id : 0,
      ref_id : Some(Uuid::new_v4().to_hyphenated().to_string()),
      parent_id : parent_id,
      parent_ref_id : None,
      description : String::from(description),
      depth : 0,
      metadata : metadata,
      source : source,
      source_id : source_id,
      created_time : 0,
      last_updated_time : 0,
      path : vec!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchAsset {
  id : u64,
  name : PatchItem,
  description : PatchItem,
  metadata : PatchItem,
  source : PatchItem,
  source_id : PatchItem,
}

impl PatchAsset {
  /// Convert an Asset to a PatchAsset which is used to update an asset.
  pub fn new(asset : &Asset) -> PatchAsset {
    PatchAsset {
      id : asset.id,
      name : PatchItem::from(&asset.name),
      description : PatchItem::from(&asset.description),
      metadata : PatchItem::from(&asset.metadata),
      source : PatchItem::from(&asset.source),
      source_id : PatchItem::from(&asset.source_id)
    }
  }
}