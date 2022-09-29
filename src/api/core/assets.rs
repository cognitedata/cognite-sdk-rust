use crate::api::resource::*;
use crate::dto::core::asset::*;
use crate::dto::items::ItemsWithCursor;
use crate::error::Result;
use crate::{Identity, Patch};

pub type Assets = Resource<Asset>;

impl WithBasePath for Assets {
    const BASE_PATH: &'static str = "assets";
}

impl List<AssetQuery, Asset> for Assets {}
impl Create<AddAsset, Asset> for Assets {}
impl<'a> SearchItems<'a, AssetFilter, AssetSearch, Asset> for Assets {}
impl Update<Patch<PatchAsset>, Asset> for Assets {}
impl DeleteWithRequest<DeleteAssetsRequest> for Assets {}
impl FilterWithRequest<FilterAssetsRequest, Asset> for Assets {}

impl Assets {
    pub async fn retrieve(
        &self,
        asset_ids: &[Identity],
        ignore_unknown_ids: bool,
        aggregated_properties: Option<Vec<String>>,
    ) -> Result<Vec<Asset>> {
        let mut id_items = RetrieveAssetsRequest::from(asset_ids);
        id_items.ignore_unknown_ids = ignore_unknown_ids;
        id_items.aggregated_properties = aggregated_properties;
        let assets_response: ItemsWithCursor<Asset> =
            self.api_client.post("assets/byids", &id_items).await?;
        Ok(assets_response.items)
    }
}
