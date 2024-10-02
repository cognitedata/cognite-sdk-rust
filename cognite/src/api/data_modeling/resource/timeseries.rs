use crate::models::{
    data_models::{CogniteTimeseries, Timeseries},
    views::ViewReference,
};

use super::{DataModelsResource, RetrieveExtendedCollection, UpsertExtendedCollection, WithView};

/// Data models files instances resource.
pub type TimeseriesResource = DataModelsResource<CogniteTimeseries>;

impl WithView for TimeseriesResource {
    const SPACE: &'static str = "cdf_cdm";
    const EXTERNAL_ID: &'static str = "CogniteTimeSeries";
    const VERSION: &'static str = "v1";

    fn view(&self) -> ViewReference {
        self.view.to_owned().unwrap_or(ViewReference {
            space: TimeseriesResource::SPACE.to_string(),
            version: TimeseriesResource::VERSION.to_string(),
            external_id: TimeseriesResource::EXTERNAL_ID.to_string(),
        })
    }
}

impl RetrieveExtendedCollection<Timeseries, CogniteTimeseries> for TimeseriesResource {}

impl UpsertExtendedCollection<CogniteTimeseries, Timeseries> for TimeseriesResource {}
