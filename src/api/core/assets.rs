use crate::api::ApiClient;
use crate::error::{Result};
use crate::dto::core::asset::*;
use crate::dto::items::Items;

pub struct Assets {
  api_client : ApiClient,
}

impl Assets {
  pub fn new(api_client : ApiClient) -> Assets {
    Assets {
      api_client : api_client
    }
  }

  pub fn filter_all(&self, asset_filter : AssetFilter) -> Result<Vec<Asset>> {
    let filter : Filter = Filter::new(asset_filter, None, None);
    match self.api_client.post::<AssetListResponse, Filter>("assets/list", &filter) {
      Ok(assets_response) => {
        Ok(assets_response.items)
      }
      Err(e) => Err(e)
    }
  }

  pub fn retrieve(&self, asset_ids : &[u64]) -> Result<Vec<Asset>> {
    let id_list : Vec<AssetId> = asset_ids.iter().map(| a_id | AssetId::from(*a_id)).collect();
    let id_items = Items::from(&id_list);
    match self.api_client.post("assets/byids", &id_items){
      Ok(result) => {
        let assets_response : AssetListResponse = result;
        let assets = assets_response.items;
        Ok(assets)
      },
      Err(e) => Err(e)
    }
  }

  pub fn search(&self, asset_filter : AssetFilter, asset_search : AssetSearch) -> Result<Vec<Asset>> {
    let filter : Search = Search::new(asset_filter, asset_search, None);
    match self.api_client.post("assets/search", &filter) {
      Ok(result) => {
        let assets_response : AssetListResponse = result;
        let assets = assets_response.items;
        Ok(assets)
      }
      Err(e) => Err(e)
    }
  }

  pub fn create(&self, assets : &[Asset]) -> Result<Vec<Asset>> {
    let add_assets : Vec<AddAsset> = assets.iter().map(| a | AddAsset::from(a)).collect();
    let add_assets_items = Items::from(&add_assets);
    match self.api_client.post("assets", &add_assets_items){
      Ok(result) => {
        let assets_response : AssetListResponse = result;
        let assets = assets_response.items;
        Ok(assets)
      },
      Err(e) => Err(e)
    }
  }

  pub fn update(&self, assets : &[Asset]) -> Result<Vec<Asset>> {
    let patch_assets : Vec<PatchAsset> = assets.iter().map(| a | PatchAsset::from(a)).collect();
    let patch_assets_items = Items::from(&patch_assets);
    match self.api_client.post("assets/update", &patch_assets_items){
      Ok(result) => {
        let assets_response : AssetListResponse = result;
        let assets = assets_response.items;
        Ok(assets)
      }
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, asset_ids : &[u64]) -> Result<()> {
    let id_list : Vec<AssetId> = asset_ids.iter().map(| a_id | AssetId::from(*a_id)).collect();
    let id_items = Items::from(&id_list);
    match self.api_client.post::<::serde_json::Value, Items>("assets/delete", &id_items){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}