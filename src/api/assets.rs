use crate::api::ApiClient;
use crate::api::params::{Params};
use crate::error::{Result};
use crate::dto::asset::*;

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

  pub fn retrieve_multiple(&self, asset_ids : &[u64]) -> Result<Vec<Asset>> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(asset_ids).unwrap());
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

  pub fn create(&self, assets : &[Asset]) -> Result<Vec<Asset>> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(assets).unwrap());
    match self.api_client.post::<AssetResponseWrapper>("assets", &request_body){
      Ok(assets_response) => {
        let assets = assets_response.data.items;
        Ok(assets)
      },
      Err(e) => Err(e)
    }
  }

  pub fn update_single(&self, asset : &Asset) -> Result<Asset> {
    let patch_asset = PatchAsset::new(asset);
    let request_body = serde_json::to_string(&patch_asset).unwrap();
    match self.api_client.post::<AssetResponseWrapper>(&format!("assets/{}/update", asset.id), &request_body){
      Ok(mut asset_response) => {
        Ok(asset_response.data.items.pop().unwrap())
      }
      Err(e) => Err(e)
    }
  }

  pub fn update(&self, assets : &[Asset]) -> Result<Vec<Asset>> {
    let patch_assets : Vec<PatchAsset> = assets.iter().map(| a | PatchAsset::new(a)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&patch_assets).unwrap());
    match self.api_client.post::<AssetResponseWrapper>("assets/update", &request_body){
      Ok(assets_response) => {
        Ok(assets_response.data.items)
      }
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, asset_ids : &[u64]) -> Result<()> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(asset_ids).unwrap());
    match self.api_client.post::<::serde_json::Value>("assets/delete", &request_body){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}