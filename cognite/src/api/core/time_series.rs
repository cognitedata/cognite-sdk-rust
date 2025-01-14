use std::collections::HashSet;
use std::iter::FromIterator;

use futures::FutureExt;

use crate::api::data_modeling::instances::Instances;
use crate::api::resource::*;
use crate::dto::core::datapoint::*;
use crate::dto::core::time_series::*;
use crate::error::Result;
use crate::get_missing_from_result;
use crate::utils::execute_with_parallelism;
use crate::Identity;
use crate::IdentityOrInstance;
use crate::IgnoreUnknownIds;
use crate::Items;
use crate::ItemsVec;
use crate::Patch;

/// A time series consists of a sequence of data points connected to a single asset.
/// For example, a water pump asset can have a temperature time series taht records a data point in
/// units of °C every second.
pub type TimeSeriesResource = Resource<TimeSeries>;

impl WithBasePath for TimeSeriesResource {
    const BASE_PATH: &'static str = "timeseries";
}

impl List<TimeSeriesQuery, TimeSeries> for TimeSeriesResource {}
impl Create<AddTimeSeries, TimeSeries> for TimeSeriesResource {}
impl FilterItems<TimeSeriesFilter, TimeSeries> for TimeSeriesResource {}
impl FilterWithRequest<TimeSeriesFilterRequest, TimeSeries> for TimeSeriesResource {}
impl SearchItems<'_, TimeSeriesFilter, TimeSeriesSearch, TimeSeries> for TimeSeriesResource {}
impl RetrieveWithIgnoreUnknownIds<Identity, TimeSeries> for TimeSeriesResource {}
impl RetrieveWithIgnoreUnknownIds<IdentityOrInstance, TimeSeries> for TimeSeriesResource {}
impl Update<Patch<PatchTimeSeries>, TimeSeries> for TimeSeriesResource {}
impl DeleteWithIgnoreUnknownIds<Identity> for TimeSeriesResource {}

impl TimeSeriesResource {
    /// Insert datapoints for a set of timeseries. Any existing datapoints with the
    /// same timestamp will be overwritten.
    ///
    /// Note: datapoints are inserted using protobuf, this converts from a slightly more ergonomic type
    /// to the protobuf types used directly in `insert_datapoints_proto`.
    ///
    /// For very performance intensive workloads, consider using `insert_datapoints_proto`
    /// directly.
    ///
    /// # Arguments
    ///
    /// * `add_datapoints` - List of datapoint batches to insert.
    pub async fn insert_datapoints(&self, add_datapoints: Vec<AddDatapoints>) -> Result<()> {
        let request = DataPointInsertionRequest::from(add_datapoints);
        self.insert_datapoints_proto(&request).await?;
        Ok(())
    }

    /// Insert datapoints for a set of timeseries. Any existing datapoints with the
    /// same timestamp will be overwritten.
    ///
    /// # Arguments
    ///
    /// * `add_datapoints` - Datapoint batches to insert.
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

    /// Insert datapoints for a set of time series, then create any missing time series.
    ///
    /// In order for this to work correctly, `generator` must return an iterator over time series
    /// with the same length as the passed slice.
    ///
    /// # Arguments
    ///
    /// * `add_datapoints` - Datapoint batches to insert.
    /// * `generator` - Method called to produce timeseries that does not exist.
    ///
    /// # Example
    ///
    /// ```ignore
    /// client.time_series.insert_datapoints_proto_create_missing(
    ///     &dps,
    ///     |idts| idts.iter().map(|idt| AddTimeSeries {
    ///         external_id: idt.as_external_id().unwrap(),
    ///         ..Default::default()
    ///     })
    /// )
    /// ```
    pub async fn insert_datapoints_proto_create_missing<T: Iterator<Item = AddDmOrTimeSeries>>(
        &self,
        add_datapoints: &DataPointInsertionRequest,
        generator: &impl Fn(&[IdentityOrInstance]) -> T,
    ) -> Result<()> {
        let result = self.insert_datapoints_proto(add_datapoints).await;
        let missing = get_missing_from_result(&result);
        let missing_idts = match missing {
            Some(m) => m,
            None => return result,
        };
        let (time_series, dm_time_series) =
            generator(&missing_idts).fold((vec![], vec![]), |mut acc, v| {
                match v {
                    AddDmOrTimeSeries::TimeSeries(add_time_series) => acc.0.push(*add_time_series),
                    AddDmOrTimeSeries::Cdm(cognite_timeseries) => acc.1.push(*cognite_timeseries),
                }
                acc
            });
        if !time_series.is_empty() {
            let futures = time_series
                .chunks(1000)
                // Since we're discarding the output, don't collect it here.
                .map(|c| self.create_ignore_duplicates(c).map(|r| r.map(|_| ())));
            execute_with_parallelism(futures, 4).await?;
        }
        if !dm_time_series.is_empty() {
            let instance_resource = Instances::new(self.api_client.clone());
            let futures = dm_time_series.chunks(1000).map(|c| {
                instance_resource
                    .apply(c, None, None, None, None, false)
                    .map(|r| r.map(|_| ()))
            });
            execute_with_parallelism(futures, 4).await?;
        }

        self.insert_datapoints_proto(add_datapoints).await
    }

    /// Insert datapoints for a set of time series, then create any missing time series.
    ///
    /// In order for this to work correctly, `generator` must return an iterator over time series
    /// with the same length as the passed slice.
    ///
    /// # Arguments
    ///
    /// * `add_datapoints` - Datapoint batches to insert.
    /// * `generator` - Method called to produce timeseries that does not exist.
    ///
    /// # Example
    ///
    /// ```ignore
    /// client.time_series.insert_datapoints_create_missing(
    ///     &dps,
    ///     |idts| idts.iter().map(|idt| AddTimeSeries {
    ///         external_id: idt.as_external_id().unwrap(),
    ///         ..Default::default()
    ///     })
    /// )
    /// ```
    pub async fn insert_datapoints_create_missing<T: Iterator<Item = AddDmOrTimeSeries>>(
        &self,
        add_datapoints: Vec<AddDatapoints>,
        generator: &impl Fn(&[IdentityOrInstance]) -> T,
    ) -> Result<()> {
        let request = DataPointInsertionRequest::from(add_datapoints);
        self.insert_datapoints_proto_create_missing(&request, generator)
            .await?;
        Ok(())
    }

    /// Insert datapoints for a set of timeseries. If the request fails due to any
    /// missing time series, remove them from the request and retry.
    ///
    /// # Arguments
    ///
    /// * `add_datapoints` - Datapoint batches to insert.
    pub async fn insert_datapoints_proto_ignore_missing(
        &self,
        add_datapoints: &DataPointInsertionRequest,
    ) -> Result<()> {
        let result = self.insert_datapoints_proto(add_datapoints).await;
        let missing = get_missing_from_result(&result);
        let missing_idts = match missing {
            Some(m) => m,
            None => return result,
        };
        let idt_set = HashSet::<IdentityOrInstance>::from_iter(missing_idts.into_iter());

        let mut items = vec![];
        for elem in add_datapoints.items.iter() {
            let idt = match &elem.time_series_reference {
                Some(x) => IdentityOrInstance::from(x.clone()),
                None => continue,
            };
            if !idt_set.contains(&idt) {
                items.push(elem.clone());
            }
        }

        if items.is_empty() {
            return Ok(());
        }

        let next_request = DataPointInsertionRequest { items };
        self.insert_datapoints_proto(&next_request).await
    }

    /// Insert datapoints for a set of timeseries. If the request fails due to any
    /// missing time series, remove them from the request and retry.
    ///
    /// # Arguments
    ///
    /// * `add_datapoints` - Datapoint batches to insert.
    pub async fn insert_datapoints_ignore_missing(
        &self,
        add_datapoints: Vec<AddDatapoints>,
    ) -> Result<()> {
        let request = DataPointInsertionRequest::from(add_datapoints);
        self.insert_datapoints_proto_ignore_missing(&request)
            .await?;
        Ok(())
    }

    /// Retrieve datapoints for a collection of time series.
    ///
    /// Note: datapoints are inserted using protobuf, this converts to a slightly more ergonomic type
    /// from the type returned by `retrieve_datapoints_proto`.
    ///
    /// For very performance intensive workloads, consider using `retrieve_datapoints_proto`
    /// directly.
    ///
    /// # Arguments
    ///
    /// * `datapoints_filter` - Filter describing which datapoints to retrieve.
    pub async fn retrieve_datapoints(
        &self,
        datapoints_filter: DatapointsFilter,
    ) -> Result<Vec<DatapointsResponse>> {
        let datapoints_response = self.retrieve_datapoints_proto(datapoints_filter).await?;
        Ok(DatapointsListResponse::from(datapoints_response).items)
    }

    /// Retrieve datapoints for a collection of time series.
    ///
    /// # Arguments
    ///
    /// * `datapoints_filter` - Filter describing which datapoints to retrieve.
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

    /// Retrieve the latest datapoint before a given time for a list of time series.
    ///
    /// # Arguments
    ///
    /// * `items` - Queries for latest datapoint.
    /// * `ignore_unknown_ids` - Set this to `true` to ignore timeseries that do not exist.
    pub async fn retrieve_latest_datapoints(
        &self,
        items: &[LatestDatapointsQuery],
        ignore_unknown_ids: bool,
    ) -> Result<Vec<LatestDatapointsResponse>> {
        let query = Items::new_with_extra_fields(items, IgnoreUnknownIds { ignore_unknown_ids });
        let datapoints_response: Items<Vec<LatestDatapointsResponse>> = self
            .api_client
            .post("timeseries/data/latest", &query)
            .await?;
        Ok(datapoints_response.items)
    }

    /// Delete ranges of datapoints for a list of time series.
    ///
    /// # Arguments
    ///
    /// * `query` - Ranges of datapoints to delete.
    pub async fn delete_datapoints(&self, query: &[DeleteDatapointsQuery]) -> Result<()> {
        let items = Items::new(query);
        self.api_client
            .post::<::serde_json::Value, _>("timeseries/data/delete", &items)
            .await?;
        Ok(())
    }

    /// Query synthetic time series. Synthetic time series lets you combine various input time series, constants,
    /// and operators, to create completely new time series.
    ///
    /// See [synthetic timeseries](https://developer.cognite.com/dev/concepts/resource_types/synthetic_timeseries.html)
    /// for more details.
    ///
    /// # Arguments
    ///
    /// * `query` - Synthetic datapoints queries.
    pub async fn query_synthetic_timeseries(
        &self,
        query: &[SyntheticTimeSeriesQuery],
    ) -> Result<Vec<SyntheticQueryResponse>> {
        let res: ItemsVec<SyntheticQueryResponse> = self
            .api_client
            .post("timeseries/synthetic/query", &Items::new(query))
            .await?;
        Ok(res.items)
    }
}
