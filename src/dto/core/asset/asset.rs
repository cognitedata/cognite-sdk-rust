use crate::dto::patch_item::PatchItem;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetListResponse {
  pub items : Vec<Asset>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
  pub id : u64,
  pub name : String,
  pub external_id: Option<String>,
  pub parent_id : Option<u64>,
  pub parent_external_id : Option<String>,
  pub description : Option<String>,
  pub depth: Option<u64>,
  pub metadata: Option<HashMap<String, String>>,
  pub source : Option<String>,
  pub last_updated_time : u128,
  pub path : Vec<u64>
}

impl Asset {
  pub fn new(name : &str, 
            description : &str, 
            external_id : Option<String>,
            parent_id : Option<u64>,
            metadata : Option<HashMap<String, String>>,
            source : Option<String>) -> Asset {
    Asset {
      name : String::from(name),
      id : 0,
      external_id : external_id,
      parent_id : parent_id,
      parent_external_id : None,
      description : Some(String::from(description)),
      depth : Some(0),
      metadata : metadata,
      source : source,
      last_updated_time : 0,
      path : vec!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddAsset {
  name: String,
  external_id: Option<String>,
  parent_id: Option<u64>,
  description: Option<String>,
  metadata: Option<HashMap<String, String>>,
  source: Option<String>,
  parent_external_id: Option<String>,
}

impl From<&Asset> for AddAsset {
  fn from(asset : &Asset) -> AddAsset {
      AddAsset { 
        name: asset.name.clone(),
        external_id: asset.external_id.clone(),
        parent_id: asset.parent_id,
        description: asset.description.clone(),
        metadata: asset.metadata.clone(),
        source: asset.source.clone(),
        parent_external_id: asset.parent_external_id.clone(),
      }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetId {
  id : u64
}

impl From<&Asset> for AssetId {
  fn from(asset : &Asset) -> AssetId {
    AssetId {
      id : asset.id
    }
  }
}

impl From<u64> for AssetId {
  fn from(asset_id : u64) -> AssetId {
    AssetId {
      id : asset_id
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchAsset {
  id : u64,
  update : PatchAssetFields
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PatchAssetFields {
  #[serde(skip_serializing_if = "Option::is_none")]
  external_id : Option<PatchItem>,
  #[serde(skip_serializing_if = "Option::is_none")]
  name : Option<PatchItem>,
  #[serde(skip_serializing_if = "Option::is_none")]
  description : Option<PatchItem>,
  #[serde(skip_serializing_if = "Option::is_none")]
  metadata : Option<PatchItem>,
  #[serde(skip_serializing_if = "Option::is_none")]
  source : Option<PatchItem>,
}

impl From<&Asset> for PatchAsset {
  fn from(asset : &Asset) -> PatchAsset {
    PatchAsset {
      id : asset.id,
      update : PatchAssetFields {
        name : PatchItem::convert(&asset.name),
        external_id : PatchItem::convert_option(&asset.external_id),
        description : PatchItem::convert_option(&asset.description),
        metadata : PatchItem::convert_option(&asset.metadata),
        source : PatchItem::convert_option(&asset.source),
      }
    }
  }
}