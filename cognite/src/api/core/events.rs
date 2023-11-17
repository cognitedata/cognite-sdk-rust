use crate::api::resource::*;
use crate::dto::core::event::*;
use crate::error::Result;
use crate::{Identity, Patch};

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
impl<'a> SearchItems<'a, EventFilter, EventSearch, Event> for EventsResource {}
impl FilterWithRequest<EventFilterQuery, Event> for EventsResource {}

impl EventsResource {
    pub async fn aggregated_count(&self, event_filter: EventFilter) -> Result<i64> {
        let filter: AggregatedEventsCountFilter = AggregatedEventsCountFilter::new(event_filter);
        let events_response: AggregatedEventCountResponse =
            self.api_client.post("events/aggregate", &filter).await?;
        Ok(events_response.items.get(0).map(|e| e.count).unwrap_or(0))
    }

    pub async fn aggregated_fields(
        &self,
        event_filter: EventFilter,
        fields: Vec<String>,
    ) -> Result<Vec<AggregatedCount>> {
        let filter: AggregatedEventsListFilter =
            AggregatedEventsListFilter::new(event_filter, fields, "uniqueValues".to_string());
        let events_response: AggregatedEventFilterResponse =
            self.api_client.post("events/aggregate", &filter).await?;
        Ok(events_response.items)
    }
}
