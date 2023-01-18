use std::collections::HashSet;
use std::iter::FromIterator;

use crate::api::resource::*;
use crate::dto::core::datapoint::*;
use crate::dto::core::time_serie::*;
use crate::error::Result;
use crate::get_missing_from_result;
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

    pub async fn insert_datapoints_proto_create_missing<T: Iterator<Item = AddTimeSerie>>(
        &self,
        add_datapoints: &DataPointInsertionRequest,
        generator: &impl Fn(&[Identity]) -> T,
    ) -> Result<()> {
        let result = self.insert_datapoints_proto(add_datapoints).await;
        let missing = get_missing_from_result(&result);
        let missing_idts = match missing {
            Some(m) => m,
            None => return result,
        };
        let to_create = generator(&missing_idts).collect::<Vec<_>>();
        self.create(&to_create).await?;

        self.insert_datapoints_proto(add_datapoints).await
    }

    pub async fn insert_datapoints_create_missing<T: Iterator<Item = AddTimeSerie>>(
        &self,
        add_datapoints: Vec<AddDatapoints>,
        generator: &impl Fn(&[Identity]) -> T,
    ) -> Result<()> {
        let request = DataPointInsertionRequest::from(add_datapoints);
        self.insert_datapoints_proto_create_missing(&request, generator)
            .await?;
        Ok(())
    }

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
        let idt_set = HashSet::<Identity>::from_iter(missing_idts.into_iter());

        let mut items = vec![];
        for elem in add_datapoints.items.iter() {
            let idt = match &elem.id_or_external_id {
                Some(x) => Identity::from(x.clone()),
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

    pub async fn insert_datapoints_ignore_missing(
        &self,
        add_datapoints: Vec<AddDatapoints>,
    ) -> Result<()> {
        let request = DataPointInsertionRequest::from(add_datapoints);
        self.insert_datapoints_proto_ignore_missing(&request)
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
