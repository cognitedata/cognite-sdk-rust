use serde::Serialize;

use crate::api::resource::*;
use crate::dto::{
    data_organization::datasets::*,
    items::{Cursor, ItemsVec},
};
use crate::error::Result;
use crate::{Filter, IdentityList, Patch};

/// API resource for data sets.
pub type DataSetsResource = Resource<DataSet>;

impl WithBasePath for DataSetsResource {
    const BASE_PATH: &'static str = "datasets";
}

impl Create<AddDataSet, DataSet> for DataSetsResource {}
impl<R> RetrieveWithIgnoreUnknownIds<IdentityList<R>, DataSet> for DataSetsResource
where
    IdentityList<R>: Serialize,
    R: Send + Sync,
{
}
impl Update<Patch<PatchDataSet>, DataSet> for DataSetsResource {}
impl FilterItems<DataSetFilter, DataSet> for DataSetsResource {}

impl DataSetsResource {
    /// Calculate the total number of data sets in the project matching the given filter
    ///
    /// # Arguments
    ///
    /// * `filter` - Optional filter.
    pub async fn count(&self, filter: DataSetFilter) -> Result<DataSetsCount> {
        let query = Filter::<DataSetFilter>::new(filter, None, None);
        let result: ItemsVec<DataSetsCount, Cursor> =
            self.api_client.post("datasets/aggregate", &query).await?;
        Ok(result.items.into_iter().next().unwrap())
    }
}
