use std::collections::HashSet;

use crate::api::resource::*;
use crate::dto::core::event::*;
use crate::error::Result;
use crate::utils::lease::CleanResource;
use crate::{Identity, ItemsVec, Patch};

/// Event objects store complex information about multiple assets over a time period.
/// Typical types of events might include Alarms, Process Data, and Logs.
///
/// For storage of low volume, manually generated, schedulable activities such as
/// maintenance schedules, work orders, or other "appointment" type activities. The Data Modelling
/// service is now recommended.
///
/// For storage of very high volume discrete events, consider using time series.
pub type EventsResource = Resource<Event>;

impl WithBasePath for EventsResource {
    const BASE_PATH: &'static str = "events";
}

impl Create<AddEvent, Event> for EventsResource {}
impl DeleteWithIgnoreUnknownIds<Identity> for EventsResource {}
impl Update<Patch<PatchEvent>, Event> for EventsResource {}
impl RetrieveWithIgnoreUnknownIds<Identity, Event> for EventsResource {}
impl SearchItems<'_, EventFilter, EventSearch, Event> for EventsResource {}
impl FilterWithRequest<EventFilterQuery, Event> for EventsResource {}

impl EventsResource {
    /// Compute aggregates over events, such as getting the count of all events in a project,
    /// checking different names and descriptions of events in your project, etc.
    ///
    /// # Arguments
    ///
    /// * `aggregate` - Aggregate to compute
    ///
    /// The returned aggregates depend on which aggregates were requested.
    pub async fn aggregate(
        &self,
        aggregate: EventAggregateRequest,
    ) -> Result<Vec<EventAggregateResponse>> {
        let resp: ItemsVec<EventAggregateResponse> =
            self.api_client.post("events/aggregate", &aggregate).await?;
        Ok(resp.items)
    }
}

impl CleanResource<Event> for EventsResource {
    async fn clean_resource(&self, resources: Vec<Event>) -> std::result::Result<(), crate::Error> {
        let ids = resources
            .iter()
            .map(|a| Identity::from(a.id))
            .collect::<HashSet<Identity>>();
        self.delete(&ids.into_iter().collect::<Vec<_>>(), true)
            .await
    }
}
