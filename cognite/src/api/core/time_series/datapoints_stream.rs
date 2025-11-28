use std::{pin::Pin, sync::Arc};

use futures::{Stream, TryStream};
use pin_project::pin_project;

use crate::{
    time_series::{DatapointAggregate, DatapointDouble, DatapointString, InstanceId},
    IdentityOrInstance,
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
    #[cfg_attr(not(test), expect(unused, reason = "Will be used in the next PR"))]
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
