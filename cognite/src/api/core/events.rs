use crate::api::resource::*;
use crate::dto::core::event::*;
use crate::error::Result;
use crate::{Identity, Patch};

pub type Events = Resource<Event>;

impl WithBasePath for Events {
    const BASE_PATH: &'static str = "events";
}

impl Create<AddEvent, Event> for Events {}
impl DeleteWithIgnoreUnknownIds<Identity> for Events {}
impl Update<Patch<PatchEvent>, Event> for Events {}
impl RetrieveWithIgnoreUnknownIds<Identity, Event> for Events {}
impl<'a> SearchItems<'a, EventFilter, EventSearch, Event> for Events {}
impl FilterWithRequest<EventFilterQuery, Event> for Events {}

impl Events {
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
