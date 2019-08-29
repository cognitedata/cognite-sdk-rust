use crate::api::ApiClient;
use crate::dto::core::asset::*;
use crate::dto::items::Items;
use crate::dto::params::Params;
use crate::error::Result;

pub struct Assets {
    api_client: ApiClient,
}

impl Assets {
    pub fn new(api_client: ApiClient) -> Assets {
        Assets {
            api_client,
        }
    }

    pub fn list(&self, params: Option<Vec<Params>>) -> Result<Vec<Asset>> {
        let assets_response: AssetListResponse =
            self.api_client.get_with_params("assets", params)?;
        Ok(assets_response.items)
    }

    pub fn filter_all(&self, asset_filter: AssetFilter) -> Result<Vec<Asset>> {
        let filter: Filter = Filter::new(asset_filter, None, None);
        let assets_response: AssetListResponse = self.api_client.post("assets/list", &filter)?;
        Ok(assets_response.items)
    }

    pub fn retrieve(&self, asset_ids: &[u64]) -> Result<Vec<Asset>> {
        let id_list: Vec<AssetId> = asset_ids.iter().map(|a_id| AssetId::from(*a_id)).collect();
        let id_items = Items::from(&id_list);
        let assets_response: AssetListResponse = self.api_client.post("assets/byids", &id_items)?;
        Ok(assets_response.items)
    }

    pub fn search(
        &self,
        asset_filter: AssetFilter,
        asset_search: AssetSearch,
    ) -> Result<Vec<Asset>> {
        let filter: Search = Search::new(asset_filter, asset_search, None);
        let assets_response: AssetListResponse = self.api_client.post("assets/search", &filter)?;
        Ok(assets_response.items)
    }

    pub fn create(&self, assets: &[Asset]) -> Result<Vec<Asset>> {
        let add_assets: Vec<AddAsset> = assets.iter().map(|a| AddAsset::from(a)).collect();
        let add_assets_items = Items::from(&add_assets);
        let assets_response: AssetListResponse =
            self.api_client.post("assets", &add_assets_items)?;
        Ok(assets_response.items)
    }

    pub fn update(&self, assets: &[Asset]) -> Result<Vec<Asset>> {
        let patch_assets: Vec<PatchAsset> = assets.iter().map(|a| PatchAsset::from(a)).collect();
        let patch_assets_items = Items::from(&patch_assets);
        let assets_response: AssetListResponse =
            self.api_client.post("assets/update", &patch_assets_items)?;
        Ok(assets_response.items)
    }

    pub fn delete(&self, asset_ids: &[u64]) -> Result<()> {
        let id_list: Vec<AssetId> = asset_ids.iter().map(|a_id| AssetId::from(*a_id)).collect();
        let id_items = Items::from(&id_list);
        self.api_client
            .post::<::serde_json::Value, Items>("assets/delete", &id_items)?;
        Ok(())
    }
}
