use std::collections::HashMap;
use super::{
  ApiClient, 
  Params,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponse {
  data : AssetListResponse
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetListResponse {
  items : Vec<Asset>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
  name : String,
  id : u64,
  parent_id : Option<u64>,
  description : String,
  metadata: Option<HashMap<String, String>>,
  source : Option<String>,
  source_id : Option<u64>,
  created_time : u64,
  last_updated_time : u64,
  path : Vec<u64>
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

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Vec<Asset> {
    let assets_response_json = self.api_client.get("assets", params).unwrap();
    let assets_response : AssetResponse = serde_json::from_str(&assets_response_json).unwrap();
    let assets = assets_response.data.items;
    assets
  }

  pub fn retrieve(&self, asset_id : u64) -> Asset {
    let http_params = None;

    let asset_response_json = self.api_client.get(&format!("assets/{}", asset_id), http_params).unwrap();
    let mut asset_response : AssetResponse = serde_json::from_str(&asset_response_json).unwrap();
    let asset = asset_response.data.items.pop().unwrap();
    asset
  }

  pub fn retrieve_multiple(&self, asset_ids : Vec<u64>) -> Vec<Asset> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&asset_ids).unwrap());
    let assets_response_json = self.api_client.post("assets/byids", &request_body).unwrap();
    let assets_response : AssetResponse = serde_json::from_str(&assets_response_json).unwrap();
    let assets = assets_response.data.items;
    assets
  }

  pub fn search(&self, params : Option<Vec<Params>>) -> Vec<Asset> {
    let assets_response_json = self.api_client.get("assets/search", params).unwrap();
    let assets_response : AssetResponse = serde_json::from_str(&assets_response_json).unwrap();
    let assets = assets_response.data.items;
    assets
  }
}