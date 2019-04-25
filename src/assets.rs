use std::collections::HashMap;
use super::{
  ApiClient, 
  Params,
  Result,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponseWrapper {
  data : AssetResponse
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponse {
  items : Vec<Asset>,
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
  pub created_time : u64,
  pub last_updated_time : u64,
  pub path : Vec<u64>
}

impl Asset {
  pub fn new(name : &str, 
            description : &str, 
            parent_id : Option<u64>,
            metadata : Option<HashMap<String, String>>) -> Asset {
    Asset {
      name : String::from(name),
      id : 0,
      ref_id : Some(Uuid::new_v4().to_hyphenated().to_string()),
      parent_id : parent_id,
      parent_ref_id : None,
      description : String::from(description),
      depth : 0,
      metadata : metadata,
      source : None,
      source_id : None,
      created_time : 0,
      last_updated_time : 0,
      path : vec!(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PatchItem {
  set : ::serde_json::Value,
  set_null : bool,
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
  fn new(asset : &Asset) -> PatchAsset {
    PatchAsset {
      id : asset.id,
      name : PatchItem { set : json!(asset.name), set_null : false },
      description : PatchItem { set : json!(asset.description), set_null : false  },
      metadata : PatchItem { set : json!(asset.metadata), set_null : asset.metadata.is_none() },
      source : PatchItem { set : json!(asset.source), set_null : asset.source.is_none() },
      source_id : PatchItem { set : json!(asset.source_id), set_null : asset.source_id.is_none() }
    }
  }
}

pub struct Assets {
  api_client : ApiClient,
}

impl Assets {
  pub fn new(api_client : ApiClient) -> Assets {
    Assets {
      api_client : api_client
    }
  }

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Result<Vec<Asset>> {
    match self.api_client.get::<AssetResponseWrapper>("assets", params) {
      Ok(assets_response) => {
        Ok(assets_response.data.items)
      }
      Err(e) => Err(e)
    }
  }

  pub fn retrieve(&self, asset_id : u64) -> Result<Asset> {
    let http_params = None;
    match self.api_client.get::<AssetResponseWrapper>(&format!("assets/{}", asset_id), http_params) {
      Ok(mut asset_response) => {
        Ok(asset_response.data.items.pop().unwrap())
      }
      Err(e) => Err(e)
    }
  }

  pub fn retrieve_multiple(&self, asset_ids : Vec<u64>) -> Result<Vec<Asset>> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&asset_ids).unwrap());
    match self.api_client.post::<AssetResponseWrapper>("assets/byids", &request_body){
      Ok(assets_response) => {
        let assets = assets_response.data.items;
        Ok(assets)
      },
      Err(e) => Err(e)
    }
  }

  pub fn search(&self, params : Option<Vec<Params>>) -> Result<Vec<Asset>> {
    match self.api_client.get::<AssetResponseWrapper>("assets/search", params) {
      Ok(assets_response) => {
        Ok(assets_response.data.items)
      }
      Err(e) => Err(e)
    }
  }

  pub fn create(&self, assets : Vec<Asset>) -> Result<Vec<Asset>> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&assets).unwrap());
    match self.api_client.post::<AssetResponseWrapper>("assets", &request_body){
      Ok(assets_response) => {
        let assets = assets_response.data.items;
        Ok(assets)
      },
      Err(e) => Err(e)
    }
  }

  pub fn update_single(&self, asset : Asset) -> Result<Asset> {
    let patch_asset = PatchAsset::new(&asset);
    let request_body = serde_json::to_string(&patch_asset).unwrap();
    match self.api_client.post::<AssetResponseWrapper>(&format!("assets/{}/update", asset.id), &request_body){
      Ok(mut asset_response) => {
        Ok(asset_response.data.items.pop().unwrap())
      }
      Err(e) => Err(e)
    }
  }

  pub fn update(&self, assets : Vec<Asset>) -> Result<Vec<Asset>> {
    let patch_assets : Vec<PatchAsset> = assets.iter().map(| a | PatchAsset::new(a)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&patch_assets).unwrap());
    match self.api_client.post::<AssetResponseWrapper>("assets/update", &request_body){
      Ok(assets_response) => {
        Ok(assets_response.data.items)
      }
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, asset_ids : Vec<u64>) -> Result<()> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&asset_ids).unwrap());
    match self.api_client.post::<::serde_json::Value>("assets/delete", &request_body){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}