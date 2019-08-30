use crate::api::ApiClient;
use crate::dto::core::event::*;
use crate::dto::items::Items;
use crate::error::{Error, Kind, Result};

pub struct Events {
    api_client: ApiClient,
}

impl Events {
    pub fn new(api_client: ApiClient) -> Events {
        Events { api_client }
    }

    pub fn create(&self, events: &[Event]) -> Result<Vec<Event>> {
        let add_events: Vec<AddEvent> = events.iter().map(AddEvent::from).collect();
        let event_items = Items::from(&add_events);
        let events_response: EventListResponse = self.api_client.post("events", &event_items)?;
        Ok(events_response.items)
    }

    pub fn filter_all(&self, event_filter: EventFilter) -> Result<Vec<Event>> {
        let filter: Filter = Filter::new(event_filter, None, None);
        let events_response: EventListResponse = self.api_client.post("events/list", &filter)?;
        Ok(events_response.items)
    }

    pub fn retrieve_single(&self, event_id: u64) -> Result<Event> {
        let mut events_response: EventListResponse =
            self.api_client.get(&format!("events/{}", event_id))?;
        if let Some(event) = events_response.items.pop() {
            return Ok(event);
        }
        Err(Error::new(Kind::NotFound("Event not found".to_owned())))
    }

    pub fn retrieve(&self, event_ids: &[u64]) -> Result<Vec<Event>> {
        let id_list: Vec<EventId> = event_ids.iter().copied().map(EventId::from).collect();
        let event_items = Items::from(&id_list);
        let events_response: EventListResponse =
            self.api_client.post("events/byids", &event_items)?;
        Ok(events_response.items)
    }

    pub fn update(&self, events: &[Event]) -> Result<Vec<Event>> {
        let patch_events: Vec<PatchEvent> = events.iter().map(PatchEvent::from).collect();
        let event_items = Items::from(&patch_events);
        let events_response: EventListResponse =
            self.api_client.post("events/update", &event_items)?;
        Ok(events_response.items)
    }

    pub fn search(
        &self,
        event_filter: EventFilter,
        event_search: EventSearch,
    ) -> Result<Vec<Event>> {
        let filter: Search = Search::new(event_filter, event_search, None);
        let events_response: EventListResponse = self
            .api_client
            .post_json("events/search", &serde_json::to_string(&filter).unwrap())?;
        Ok(events_response.items)
    }

    pub fn delete(&self, event_ids: &[u64]) -> Result<()> {
        let id_list: Vec<EventId> = event_ids.iter().copied().map(EventId::from).collect();
        let event_items = Items::from(&id_list);
        self.api_client
            .post::<::serde_json::Value, Items>("events/delete", &event_items)?;
        Ok(())
    }
}
