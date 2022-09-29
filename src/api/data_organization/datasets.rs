use crate::api::resource::*;
use crate::dto::{data_organization::datasets::*, items::ItemsWithCursor};
use crate::error::Result;
use crate::{Filter, Identity, Patch};

pub type DataSets = Resource<DataSet>;

impl WithBasePath for DataSets {
    const BASE_PATH: &'static str = "datasets";
}

impl Create<AddDataSet, DataSet> for DataSets {}
impl RetrieveWithIgnoreUnknownIds<Identity, DataSet> for DataSets {}
impl Update<Patch<PatchDataSet>, DataSet> for DataSets {}
impl FilterItems<DataSetFilter, DataSet> for DataSets {}

impl DataSets {
    pub async fn count(&self, filter: DataSetFilter) -> Result<DataSetsCount> {
        let query = Filter::<DataSetFilter>::new(filter, None, None);
        let result: ItemsWithCursor<DataSetsCount> =
            self.api_client.post("datasets/aggregate", &query).await?;
        Ok(result.items.into_iter().next().unwrap())
    }
}
