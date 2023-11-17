use crate::api::resource::*;
use crate::dto::{data_organization::datasets::*, items::ItemsWithCursor};
use crate::error::Result;
use crate::{Filter, Identity, Patch};

/// Data sets let you document and track data lineage, as well as
/// restrict access to data.
///
/// Data sets group and track data by its source.
/// For example, a data set can contain all work orders originating from SAP.
/// Typically, an organization will have one data set for each of its data ingestion pipelines in CDF.
pub type DataSetsResource = Resource<DataSet>;

impl WithBasePath for DataSetsResource {
    const BASE_PATH: &'static str = "datasets";
}

impl Create<AddDataSet, DataSet> for DataSetsResource {}
impl RetrieveWithIgnoreUnknownIds<Identity, DataSet> for DataSetsResource {}
impl Update<Patch<PatchDataSet>, DataSet> for DataSetsResource {}
impl FilterItems<DataSetFilter, DataSet> for DataSetsResource {}

impl DataSetsResource {
    pub async fn count(&self, filter: DataSetFilter) -> Result<DataSetsCount> {
        let query = Filter::<DataSetFilter>::new(filter, None, None);
        let result: ItemsWithCursor<DataSetsCount> =
            self.api_client.post("datasets/aggregate", &query).await?;
        Ok(result.items.into_iter().next().unwrap())
    }
}
