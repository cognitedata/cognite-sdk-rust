use std::collections::HashMap;
use super::{
  ApiClient, 
  Params,
  Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponseWrapper {
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

  pub fn create(&self, assets : Vec<Asset>) -> Result<Asset> {
    unimplemented!();
  }

  pub fn update_single(&self, asset : Asset) -> Result<Asset> {
    unimplemented!();
  }

  pub fn update(&self, assets : Vec<Asset>) -> Result<Vec<Asset>> {
    unimplemented!();
  }

  pub fn delete(&self, asset_ids : Vec<u64>) -> Result<()> {
    unimplemented!();
  }
}