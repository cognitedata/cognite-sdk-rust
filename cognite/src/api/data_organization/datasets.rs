use crate::api::resource::*;
use crate::dto::{data_organization::datasets::*, items::ItemsVec};
use crate::error::Result;
use crate::{Filter, Identity, Patch};

/// API resource for data sets.
pub type DataSetsResource = Resource<DataSet>;

impl WithBasePath for DataSetsResource {
    const BASE_PATH: &'static str = "datasets";
}

impl Create<AddDataSet, DataSet> for DataSetsResource {}
impl RetrieveWithIgnoreUnknownIds<Identity, DataSet> for DataSetsResource {}
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
        let result: ItemsVec<DataSetsCount> =
            self.api_client.post("datasets/aggregate", &query).await?;
        Ok(result.items.into_iter().next().unwrap())
    }
}
