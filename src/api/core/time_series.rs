use crate::api::resource::*;
use crate::dto::core::datapoint::*;
use crate::dto::core::time_serie::*;
use crate::error::Result;
use crate::Identity;
use crate::Items;
use crate::ItemsWithIgnoreUnknownIds;
use crate::Patch;

pub type TimeSeries = Resource<TimeSerie>;

impl WithBasePath for TimeSeries {
    const BASE_PATH: &'static str = "timeseries";
}

impl List<TimeSerieQuery, TimeSerie> for TimeSeries {}
impl Create<AddTimeSerie, TimeSerie> for TimeSeries {}
impl FilterItems<TimeSerieFilter, TimeSerie> for TimeSeries {}
impl<'a> SearchItems<'a, TimeSerieFilter, TimeSerieSearch, TimeSerie> for TimeSeries {}
impl RetrieveWithIgnoreUnknownIds<Identity, TimeSerie> for TimeSeries {}
impl Update<Patch<PatchTimeSerie>, TimeSerie> for TimeSeries {}
impl DeleteWithIgnoreUnknownIds<Identity> for TimeSeries {}

impl TimeSeries {
    pub async fn insert_datapoints(&self, add_datapoints: Vec<AddDatapoints>) -> Result<()> {
        let request = DataPointInsertionRequest::from(add_datapoints);
        self.insert_datapoints_proto(&request).await?;
        Ok(())
    }

    pub async fn insert_datapoints_proto(
        &self,
        add_datapoints: &DataPointInsertionRequest,
    ) -> Result<()> {
        self.api_client
            .post_protobuf::<::serde_json::Value, DataPointInsertionRequest>(
                "timeseries/data",
                add_datapoints,
            )
            .await?;
        Ok(())
    }

    pub async fn retrieve_datapoints(
        &self,
        datapoints_filter: DatapointsFilter,
    ) -> Result<Vec<DatapointsResponse>> {
        let datapoints_response = self.retrieve_datapoints_proto(datapoints_filter).await?;
        Ok(DatapointsListResponse::from(datapoints_response).items)
    }

    pub async fn retrieve_datapoints_proto(
        &self,
        datapoints_filter: DatapointsFilter,
    ) -> Result<DataPointListResponse> {
        let datapoints_response: DataPointListResponse = self
            .api_client
            .post_expect_protobuf("timeseries/data/list", &datapoints_filter)
            .await?;
        Ok(datapoints_response)
    }

    pub async fn retrieve_latest_datapoints(
        &self,
        items: &[LatestDatapointsQuery],
        ignore_unknown_ids: bool,
    ) -> Result<Vec<DatapointsResponse>> {
        let query = ItemsWithIgnoreUnknownIds::new(items, ignore_unknown_ids);
        let datapoints_response: DatapointsListResponse = self
            .api_client
            .post("timeseries/data/latest", &query)
            .await?;
        Ok(datapoints_response.items)
    }

    pub async fn delete_datapoints(&self, query: &[DeleteDatapointsQuery]) -> Result<()> {
        let items = Items::from(query);
        self.api_client
            .post::<::serde_json::Value, Items>("timeseries/data/delete", &items)
            .await?;
        Ok(())
    }
}
