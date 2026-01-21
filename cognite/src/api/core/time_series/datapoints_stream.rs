use std::{
    collections::{HashMap, VecDeque},
    pin::Pin,
    sync::Arc,
};

use futures::{stream::FuturesUnordered, Stream, StreamExt, TryStream};
use pin_project::pin_project;

use crate::{
    time_series::{
        DataPointListItem, DataPointListResponse, DatapointAggregate, DatapointDouble,
        DatapointString, DatapointsFilter, DatapointsQuery, InstanceId, ListDatapointType,
        TimeSeriesResource,
    },
    Identity, IdentityOrInstance,
};

/// A datapoint of either type.
pub enum EitherDataPoint {
    /// A numeric datapoint.
    Numeric(DatapointDouble),
    /// A string datapoint.
    String(DatapointString),
    /// An aggregate datapoint.
    Aggregate(DatapointAggregate),
}

struct TimeSeriesRef {
    id: i64,
    external_id: Option<String>,
    instance_id: Option<InstanceId>,
    original_id: IdentityOrInstance,
    is_string: bool,
    is_step: bool,
    unit: Option<String>,
    unit_external_id: Option<String>,
}

/// A datapoint containing a reference to its timeseries metadata.
/// Used in streaming responses to avoid cloning timeseries info for every datapoint.
pub struct DataPointRef {
    // This is an Arc to avoid cloning the timeseries information for every datapoint,
    // which can be a considerable amount of data for larger requests.
    timeseries: Arc<TimeSeriesRef>,
    datapoint: EitherDataPoint,
}

impl DataPointRef {
    /// Get the internal ID of the timeseries this datapoint belongs to.
    pub fn id(&self) -> i64 {
        self.timeseries.id
    }

    /// Get the external ID of the timeseries this datapoint belongs to, if it has one.
    pub fn external_id(&self) -> Option<&str> {
        self.timeseries.external_id.as_deref()
    }

    /// Get the data modelling instance ID of the timeseries this datapoint belongs to, if it has one.
    pub fn instance_id(&self) -> Option<&InstanceId> {
        self.timeseries.instance_id.as_ref()
    }

    /// Get the original ID used to identify the timeseries this datapoint belongs to in the request.
    pub fn original_id(&self) -> &IdentityOrInstance {
        &self.timeseries.original_id
    }

    /// Check if the timeseries this datapoint belongs to is of string type.
    pub fn is_string(&self) -> bool {
        self.timeseries.is_string
    }

    /// Check if the timeseries this datapoint belongs to is a step timeseries.
    pub fn is_step(&self) -> bool {
        self.timeseries.is_step
    }

    /// Get the unit of the timeseries this datapoint belongs to, if it has one.
    pub fn unit(&self) -> Option<&str> {
        self.timeseries.unit.as_deref()
    }

    /// Get the external ID of the unit of the timeseries this datapoint belongs to, if it has one.
    pub fn unit_external_id(&self) -> Option<&str> {
        self.timeseries.unit_external_id.as_deref()
    }

    /// Consume the reference and return the underlying datapoint, to avoid cloning.
    pub fn into_datapoint(self) -> EitherDataPoint {
        self.datapoint
    }

    /// Get a reference to the underlying datapoint.
    pub fn datapoint(&self) -> &EitherDataPoint {
        &self.datapoint
    }

    /// Get a reference to the underlying datapoint as numeric, if it is of that type.
    pub fn as_numeric(&self) -> Option<&DatapointDouble> {
        match &self.datapoint {
            EitherDataPoint::Numeric(dp) => Some(dp),
            _ => None,
        }
    }

    /// Get a reference to the underlying datapoint as string, if it is of that type.
    pub fn as_string(&self) -> Option<&DatapointString> {
        match &self.datapoint {
            EitherDataPoint::String(dp) => Some(dp),
            _ => None,
        }
    }

    /// Get a reference to the underlying datapoint as aggregate, if it is of that type.
    pub fn as_aggregate(&self) -> Option<&DatapointAggregate> {
        match &self.datapoint {
            EitherDataPoint::Aggregate(dp) => Some(dp),
            _ => None,
        }
    }
}

struct FetchResult {
    query_items: Vec<DatapointsQuery>,
    response: DataPointListResponse,
}

/// Options for configuring the behavior of a `DatapointsStream`.
#[derive(Clone, Debug)]
pub struct DatapointsStreamOptions {
    /// The maximum number of timeseries to include in each request. Default is 100.
    pub batch_size: usize,
    /// The maximum number of requests to have in flight at any given time. Default is 4.
    pub parallelism: usize,
}

impl Default for DatapointsStreamOptions {
    fn default() -> Self {
        Self {
            batch_size: 100,
            parallelism: 4,
        }
    }
}

#[cfg(target_arch = "wasm32")]
type LBoxFuture<'a, T> = futures::future::LocalBoxFuture<'a, T>;

#[cfg(not(target_arch = "wasm32"))]
type LBoxFuture<'a, T> = futures::future::BoxFuture<'a, T>;

pub(super) struct DatapointsStream<'a> {
    timeseries: &'a TimeSeriesResource,
    filter: DatapointsFilter,
    queries: VecDeque<DatapointsQuery>,
    options: DatapointsStreamOptions,
    known_timeseries: HashMap<i64, Arc<TimeSeriesRef>>,
    // Technically, if we had existential types, we could avoid the box here.
    // In practice it really doesn't matter, the overhead of a network request is much larger
    // than anything from boxing.
    futures: FuturesUnordered<LBoxFuture<'a, Result<FetchResult, crate::Error>>>,
}

impl<'a> DatapointsStream<'a> {
    pub(super) fn new(
        timeseries: &'a TimeSeriesResource,
        mut filter: DatapointsFilter,
        options: DatapointsStreamOptions,
    ) -> Self {
        Self {
            timeseries,
            queries: std::mem::take(&mut filter.items).into(),
            filter,
            options,
            known_timeseries: HashMap::new(),
            futures: FuturesUnordered::new(),
        }
    }

    async fn fetch_batch(
        timeseries: &'a TimeSeriesResource,
        filter: DatapointsFilter,
    ) -> Result<FetchResult, crate::Error> {
        let response = timeseries.retrieve_datapoints_proto(&filter).await?;
        Ok(FetchResult {
            query_items: filter.items,
            response,
        })
    }

    fn update_known_timeseries_from_batch(
        &mut self,
        response: &DataPointListItem,
        query: &DatapointsQuery,
    ) {
        // We've already seen this timeseries, nothing to do.
        if self.known_timeseries.contains_key(&response.id) {
            return;
        }

        self.known_timeseries.insert(
            response.id,
            Arc::new(TimeSeriesRef {
                id: response.id,
                external_id: if !response.external_id.is_empty() {
                    Some(response.external_id.clone())
                } else {
                    None
                },
                instance_id: response.instance_id.clone(),
                original_id: query.id.clone(),
                is_string: response.is_string,
                is_step: response.is_step,
                unit: if !response.unit.is_empty() {
                    Some(response.unit.clone())
                } else {
                    None
                },
                unit_external_id: if !response.unit_external_id.is_empty() {
                    Some(response.unit_external_id.clone())
                } else {
                    None
                },
            }),
        );
    }

    fn equals_identity(id: &IdentityOrInstance, response_item: &DataPointListItem) -> bool {
        match id {
            IdentityOrInstance::Identity(Identity::Id { id }) => response_item.id == *id,
            IdentityOrInstance::Identity(Identity::ExternalId { external_id }) => {
                response_item.external_id == *external_id
            }
            IdentityOrInstance::InstanceId { instance_id } => response_item
                .instance_id
                .as_ref()
                .is_some_and(|i| i == instance_id),
        }
    }

    async fn stream_batches_inner(
        &mut self,
        maintain_internal_state: bool,
    ) -> Result<Option<DataPointListResponse>, crate::Error> {
        // If there's room for more requests, spawn them immediately.
        while self.futures.len() < self.options.parallelism && !self.queries.is_empty() {
            let mut batch = Vec::with_capacity(self.options.batch_size.min(self.queries.len()));
            while batch.len() < self.options.batch_size {
                if let Some(query) = self.queries.pop_front() {
                    batch.push(query);
                } else {
                    break;
                }
            }
            let filter = DatapointsFilter {
                items: batch,
                ..self.filter.clone()
            };
            let timeseries = self.timeseries;
            self.futures
                .push(Box::pin(Self::fetch_batch(timeseries, filter)));
        }

        // Wait for the next request to complete.
        let Some(result) = self.futures.next().await else {
            // No more requests in flight, we're done.
            return Ok(None);
        };
        let mut fetch_result = result?;
        let mut query_iter = fetch_result.query_items.into_iter();

        // Update queries from the result, then re-queue them.
        for response_item in &mut fetch_result.response.items {
            // Datapoints are returned in order, so we check if the next query has an ID matching the
            // response item. If not, we keep going until we find it.
            let Some(mut query) = query_iter.next() else {
                return Err(crate::Error::Other(
                    "Internal logic error: more response items than query items".to_string(),
                ));
            };

            while !Self::equals_identity(&query.id, response_item) {
                // This query had no datapoints in the response, so we don't need to do anything
                // special with it. Just move on to the next one.
                let Some(next_query) = query_iter.next() else {
                    return Err(crate::Error::Other(
                        "Internal logic error: response item does not match any query item"
                            .to_string(),
                    ));
                };
                query = next_query;
            }

            // If we're maintaining internal state, record information about
            // the timeseries we've seen.
            if maintain_internal_state {
                self.update_known_timeseries_from_batch(response_item, &query);
            }

            if !response_item.next_cursor.is_empty() {
                // Take the cursor. There's no reason to allow the caller to
                // see it or use it.
                query.cursor = Some(std::mem::take(&mut response_item.next_cursor));
                self.queries.push_back(query);
            }
        }

        Ok(Some(fetch_result.response))
    }

    pub fn stream_batches(
        self,
    ) -> impl Stream<Item = Result<DataPointListResponse, crate::Error>> + 'a {
        futures::stream::try_unfold(self, move |mut state| async move {
            Ok(state.stream_batches_inner(false).await?.map(|v| (v, state)))
        })
    }

    pub fn stream_datapoints(self) -> impl Stream<Item = Result<DataPointRef, crate::Error>> + 'a {
        FlatIterStream::new(futures::stream::try_unfold(
            self,
            move |mut state| async move {
                let Some(batch) = state.stream_batches_inner(true).await? else {
                    return Ok(None);
                };
                let mut res = Vec::new();

                for item in batch.items {
                    let timeseries = state
                        .known_timeseries
                        .get(&item.id)
                        .ok_or_else(|| {
                            crate::Error::Other(format!(
                                "Internal logic error: timeseries with id {} not found in known_timeseries",
                                item.id
                            ))
                        })?
                        .clone();
                    match item.datapoint_type {
                        None => continue,
                        Some(ListDatapointType::AggregateDatapoints(dps)) => {
                            res.extend(dps.datapoints.into_iter().map(move |dp| DataPointRef {
                                timeseries: timeseries.clone(),
                                datapoint: EitherDataPoint::Aggregate(dp.into()),
                            }));
                        }
                        Some(ListDatapointType::StringDatapoints(dps)) => {
                            res.extend(dps.datapoints.into_iter().map(move |dp| DataPointRef {
                                timeseries: timeseries.clone(),
                                datapoint: EitherDataPoint::String(dp.into()),
                            }));
                        }
                        Some(ListDatapointType::NumericDatapoints(dps)) => {
                            res.extend(dps.datapoints.into_iter().map(move |dp| DataPointRef {
                                timeseries: timeseries.clone(),
                                datapoint: EitherDataPoint::Numeric(dp.into()),
                            }));
                        }
                    }
                }

                Ok(Some((res.into_iter(), state)))
            },
        ))
    }
}

#[pin_project]
/// Simple stream adapter that flattens a stream of iterables into a stream of items.
struct FlatIterStream<R>
where
    R: TryStream,
    R::Ok: IntoIterator,
{
    #[pin]
    inner: R,
    current: Option<<R::Ok as IntoIterator>::IntoIter>,
}

impl<R> FlatIterStream<R>
where
    R: TryStream,
    R::Ok: IntoIterator,
{
    fn new(stream: R) -> Self {
        Self {
            inner: stream,
            current: None,
        }
    }
}

impl<R: TryStream> Stream for FlatIterStream<R>
where
    R: TryStream,
    R::Ok: IntoIterator,
{
    type Item = Result<<R::Ok as IntoIterator>::Item, R::Error>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut this = self.project();
        loop {
            if let Some(current) = this.current.as_mut() {
                if let Some(item) = current.next() {
                    return std::task::Poll::Ready(Some(Ok(item)));
                } else {
                    *this.current = None;
                }
            }
            match this.inner.as_mut().try_poll_next(cx)? {
                std::task::Poll::Ready(Some(next_iter)) => {
                    *this.current = Some(next_iter.into_iter());
                }
                std::task::Poll::Ready(None) => return std::task::Poll::Ready(None),
                std::task::Poll::Pending => return std::task::Poll::Pending,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::core::time_series::datapoints_stream::FlatIterStream, time_series::StatusCode,
    };
    #[test]
    fn test_datapoint_ref() {
        use super::*;
        let ts_ref = Arc::new(TimeSeriesRef {
            id: 42,
            external_id: Some("ts1".to_string()),
            instance_id: None,
            original_id: IdentityOrInstance::from("ts1"),
            is_string: false,
            is_step: false,
            unit: Some("°C".to_string()),
            unit_external_id: None,
        });
        let dp = DataPointRef {
            timeseries: ts_ref.clone(),
            datapoint: EitherDataPoint::Numeric(DatapointDouble {
                timestamp: 1625079600000,
                value: Some(23.5),
                status: Some(StatusCode::Good),
            }),
        };
        assert_eq!(dp.id(), 42);
        assert_eq!(dp.external_id(), Some("ts1"));
        assert!(!dp.is_string());
        assert_eq!(dp.unit(), Some("°C"));
        if let EitherDataPoint::Numeric(n) = dp.datapoint() {
            assert_eq!(n.value, Some(23.5));
        } else {
            panic!("Expected numeric datapoint");
        }
    }
    #[test]
    fn test_flat_iter_stream() {
        use futures::stream;
        use futures::StreamExt;
        let s = stream::iter(vec![
            Ok::<_, crate::Error>(vec![1, 2, 3]),
            Ok(vec![4, 5]),
            Ok(vec![6]),
        ]);
        let mut flat_stream = FlatIterStream::new(s);
        let mut results = Vec::new();
        while let Some(item) = futures::executor::block_on(flat_stream.next()) {
            results.push(item.unwrap());
        }
        assert_eq!(results, vec![1, 2, 3, 4, 5, 6]);
    }
}
