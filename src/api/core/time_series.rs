use crate::api::ApiClient;
use crate::dto::core::datapoint::*;
use crate::dto::core::time_serie::*;
use crate::dto::items::Items;
use crate::dto::params::Params;
use crate::error::{Error, Kind, Result};

pub struct TimeSeries {
    api_client: ApiClient,
}

impl TimeSeries {
    pub fn new(api_client: ApiClient) -> TimeSeries {
        TimeSeries { api_client }
    }

    pub fn list_all(&self, params: Option<Vec<Params>>) -> Result<Vec<TimeSerie>> {
        let time_series_response: TimeSerieListResponse =
            self.api_client.get_with_params("timeseries", params)?;
        Ok(time_series_response.items)
    }

    pub fn create(&self, time_series: &[TimeSerie]) -> Result<Vec<TimeSerie>> {
        let add_time_series: Vec<AddTimeSerie> =
            time_series.iter().map(AddTimeSerie::from).collect();
        let add_time_series_items = Items::from(&add_time_series);
        let time_series_response: TimeSerieListResponse =
            self.api_client.post("timeseries", &add_time_series_items)?;
        Ok(time_series_response.items)
    }

    pub fn search(
        &self,
        time_serie_filter: TimeSerieFilter,
        time_serie_search: TimeSerieSearch,
    ) -> Result<Vec<TimeSerie>> {
        let filter: Search = Search::new(time_serie_filter, time_serie_search, None);
        let time_series_response: TimeSerieListResponse =
            self.api_client.post("timeseries/search", &filter)?;
        Ok(time_series_response.items)
    }

    pub fn retrieve(&self, time_serie_ids: &[u64]) -> Result<Vec<TimeSerie>> {
        let id_list: Vec<TimeSerieId> = time_serie_ids
            .iter()
            .copied()
            .map(TimeSerieId::from)
            .collect();
        let id_items = Items::from(&id_list);
        let time_series_response: TimeSerieListResponse =
            self.api_client.post("timeseries/byids", &id_items)?;
        Ok(time_series_response.items)
    }

    pub fn update(&self, time_series: &[TimeSerie]) -> Result<Vec<TimeSerie>> {
        let patch_time_series: Vec<PatchTimeSerie> =
            time_series.iter().map(PatchTimeSerie::from).collect();
        let patch_time_series_items = Items::from(&patch_time_series);
        let time_series_response: TimeSerieListResponse = self
            .api_client
            .post("timeseries/update", &patch_time_series_items)?;
        Ok(time_series_response.items)
    }

    pub fn delete(&self, time_serie_ids: &[u64]) -> Result<()> {
        let id_list: Vec<TimeSerieId> = time_serie_ids
            .iter()
            .copied()
            .map(TimeSerieId::from)
            .collect();
        let id_items = Items::from(&id_list);
        self.api_client
            .post::<::serde_json::Value, Items>("timeseries/delete", &id_items)?;
        Ok(())
    }

    pub fn insert_datapoints(&self, add_datapoints: &[AddDatapoints]) -> Result<()> {
        let add_datapoints_items = Items::from(add_datapoints);
        self.api_client
            .post::<::serde_json::Value, Items>("timeseries/data", &add_datapoints_items)?;
        Ok(())
    }

    pub fn retrieve_datapoints(
        &self,
        datapoints_filter: DatapointsFilter,
    ) -> Result<Vec<DatapointsResponse>> {
        let datapoints_response: DatapointsListResponse = self
            .api_client
            .post("timeseries/data/list", &datapoints_filter)?;
        Ok(datapoints_response.items)
    }

    pub fn retrieve_latest_datapoints(
        &self,
        time_serie_id: u64,
        before: &str,
    ) -> Result<DatapointsResponse> {
        let latest_datapoint_query: LatestDatapointsQuery =
            LatestDatapointsQuery::new(time_serie_id, before);
        let mut datapoints_response: DatapointsListResponse = self
            .api_client
            .post("timeseries/data/latest", &latest_datapoint_query)?;
        if let Some(datapoint) = datapoints_response.items.pop() {
            return Ok(datapoint);
        }
        Err(Error::new(Kind::NotFound("Datapoint not found".to_owned())))
    }

    pub fn delete_datapoints(
        &self,
        time_serie_id: u64,
        inclusive_begin: i64,
        exclusive_end: i64,
    ) -> Result<()> {
        let delete_datapoint_query: DeleteDatapointsQuery =
            DeleteDatapointsQuery::new(time_serie_id, inclusive_begin, exclusive_end);
        self.api_client
            .post::<::serde_json::Value, DeleteDatapointsQuery>(
                "timeseries/data/delete",
                &delete_datapoint_query,
            )?;
        Ok(())
    }
}
