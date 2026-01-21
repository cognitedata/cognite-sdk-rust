use serde::Serialize;
use std::collections::HashSet;

use crate::api::resource::*;
use crate::dto::core::asset::*;
use crate::error::Result;
use crate::utils::lease::CleanResource;
use crate::{IdentityList, ItemsVec, Patch};

/// Assets represent objects or groups of objects from the physical world.
/// Assets are organized in hierarchies. For example, a water pump asset can
/// be part of a subsystem asset on an oil platform asset.
pub type AssetsResource = Resource<Asset>;

impl WithBasePath for AssetsResource {
    const BASE_PATH: &'static str = "assets";
}

impl List<AssetQuery, Asset> for AssetsResource {}
impl Create<AddAsset, Asset> for AssetsResource {}
impl SearchItems<'_, AssetFilter, AssetSearch, Asset> for AssetsResource {}
impl Update<Patch<PatchAsset>, Asset> for AssetsResource {}
impl<R> DeleteWithRequest<DeleteAssetsRequest<IdentityList<R>>> for AssetsResource
where
    R: Send + Sync,
    IdentityList<R>: Serialize,
{
}
impl FilterWithRequest<FilterAssetsRequest, Asset> for AssetsResource {}
impl<R> RetrieveWithRequest<RetrieveAssetsRequest<IdentityList<R>>, ItemsVec<Asset>>
    for AssetsResource
where
    R: Send + Sync,
    IdentityList<R>: Serialize,
{
}

impl AssetsResource {
    /// Retrieve a list of assets by their IDs.
    ///
    /// Will fail if `ignore_unknown_ids` is false and the assets are not present in CDF.
    ///
    /// # Arguments
    ///
    /// * `asset_ids` - List of IDs or external IDs to retrieve.
    /// * `ignore_unknown_ids` - If `true`, missing assets will be ignored, instead of causing
    ///   the request to fail.
    /// * `aggregated_properties` - List of aggregated properties to include in response.
    pub async fn retrieve<R>(
        &self,
        asset_ids: impl Into<IdentityList<R>>,
        ignore_unknown_ids: bool,
        aggregated_properties: Option<Vec<AssetAggregatedProperty>>,
    ) -> Result<Vec<Asset>>
    where
        IdentityList<R>: Serialize,
        R: Send + Sync,
    {
        let id_items = RetrieveAssetsRequest::new_with_extra_fields(
            asset_ids.into(),
            RetrieveAssetsRequestData {
                ignore_unknown_ids,
                aggregated_properties,
            },
        );
        let r = RetrieveWithRequest::retrieve(self, &id_items).await?;
        Ok(r.items)
    }

    /// Delete a list of assets by their IDs.
    ///
    /// Will fail if `ignore_unknown_ids` is false and the assets are not present in CDF.
    ///
    /// # Arguments
    /// * `asset_ids` - List of IDs or external IDs to delete.
    /// * `ignore_unknown_ids` - If `true`, missing assets will be ignored, instead of causing
    ///   the request to fail.
    /// * `recursive` - If `true`, recursively delete any children of the deleted assets.
    pub async fn delete<R>(
        &self,
        asset_ids: impl Into<IdentityList<R>>,
        ignore_unknown_ids: bool,
        recursive: bool,
    ) -> Result<()>
    where
        IdentityList<R>: Serialize,
        R: Send + Sync,
    {
        let id_items = DeleteAssetsRequest::new_with_extra_fields(
            asset_ids.into(),
            DeleteAssetsRequestData {
                ignore_unknown_ids,
                recursive,
            },
        );
        DeleteWithRequest::delete(self, &id_items).await
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

impl CleanResource<Asset> for AssetsResource {
    async fn clean_resource(&self, resources: Vec<Asset>) -> std::result::Result<(), crate::Error> {
        let ids = resources.iter().map(|a| a.id).collect::<HashSet<i64>>();
        self.delete(&ids.into_iter().collect::<Vec<_>>(), true, true)
            .await
    }
}
