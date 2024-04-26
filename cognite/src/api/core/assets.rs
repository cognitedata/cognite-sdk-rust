use crate::api::resource::*;
use crate::dto::core::asset::*;
use crate::error::Result;
use crate::{Identity, ItemsVec, Patch};

/// Assets represent objects or groups of objects from the physical world.
/// Assets are organized in hierarchies. For example, a water pump asset can
/// be part of a subsystem asset on an oil platform asset.
pub type AssetsResource = Resource<Asset>;

impl WithBasePath for AssetsResource {
    const BASE_PATH: &'static str = "assets";
}

impl List<AssetQuery, Asset> for AssetsResource {}
impl Create<AddAsset, Asset> for AssetsResource {}
impl<'a> SearchItems<'a, AssetFilter, AssetSearch, Asset> for AssetsResource {}
impl Update<Patch<PatchAsset>, Asset> for AssetsResource {}
impl DeleteWithRequest<DeleteAssetsRequest> for AssetsResource {}
impl FilterWithRequest<FilterAssetsRequest, Asset> for AssetsResource {}
impl RetrieveWithRequest<RetrieveAssetsRequest, Asset> for AssetsResource {}

impl AssetsResource {
    /// Retrieve a list of assets by their IDs.
    ///
    /// Will fail if `ignore_unknown_ids` is false and the assets are not present in CDF.
    ///
    /// # Arguments
    ///
    /// * `asset_ids` - List of IDs or external IDs to retrieve.
    /// * `ignore_unknown_ids` - If `true`, missing assets will be ignored, instead of causing
    /// the request to fail.
    /// * `aggregated_properties` - List of aggregated properties to include in response.
    pub async fn retrieve(
        &self,
        asset_ids: &[Identity],
        ignore_unknown_ids: bool,
        aggregated_properties: Option<Vec<AssetAggregatedProperty>>,
    ) -> Result<Vec<Asset>> {
        let mut id_items = RetrieveAssetsRequest::from(asset_ids);
        id_items.ignore_unknown_ids = ignore_unknown_ids;
        id_items.aggregated_properties = aggregated_properties;
        let assets_response: ItemsVec<Asset> =
            self.api_client.post("assets/byids", &id_items).await?;
        Ok(assets_response.items)
    }

    /// Compute aggregates over assets, such as getting the count of all assets in a project,
    /// checking different names and descriptions of assets in your project, etc.
    ///
    /// # Arguments
    ///
    /// * `aggregate` - Aggregate to compute
    ///
    /// The returned aggregates depend on which aggregates were requested.
    pub async fn aggregate(
        &self,
        aggregate: AssetAggregateRequest,
    ) -> Result<Vec<AssetAggregateResponse>> {
        let resp: ItemsVec<AssetAggregateResponse> =
            self.api_client.post("assets/aggregate", &aggregate).await?;
        Ok(resp.items)
    }
}
