use std::sync::Arc;

use cognite::events::{AddEvent, Event, EventFilter, EventFilterQuery, EventSearch, PatchEvent};
use cognite::{Cursor, Identity, ItemsVec, Patch};

use crate::client::MockClient;
use crate::error::{MockError, Result};
use crate::patch::{apply_list_i64, apply_map, apply_set_null, paginate};

const DEFAULT_LIMIT: usize = 1000;

pub struct MockEventsResource {
    pub(crate) client: Arc<MockClient>,
}

impl MockEventsResource {
    pub fn new(client: Arc<MockClient>) -> Self {
        Self { client }
    }

    pub async fn create(&self, creates: &[AddEvent]) -> Result<Vec<Event>> {
        let mut store = self.client.events.write().await;
        let mut result = Vec::with_capacity(creates.len());
        for add in creates {
            if let Some(ext) = &add.external_id {
                if store.by_ext.contains_key(ext.as_str()) {
                    return Err(MockError::AlreadyExists(format!(
                        "Event with externalId '{}' already exists",
                        ext
                    )));
                }
            }
            let id = self.client.id_gen.next();
            let event = Event {
                id,
                external_id: add.external_id.clone(),
                data_set_id: add.data_set_id,
                start_time: add.start_time,
                end_time: add.end_time,
                r#type: add.r#type.clone(),
                subtype: add.subtype.clone(),
                description: add.description.clone(),
                metadata: add.metadata.clone(),
                asset_ids: add.asset_ids.clone(),
                source: add.source.clone(),
                created_time: 0,
                last_updated_time: 0,
            };
            store.insert(event.clone());
            result.push(event);
        }
        Ok(result)
    }

    pub async fn retrieve(&self, ids: &[Identity], ignore_unknown_ids: bool) -> Result<Vec<Event>> {
        let store = self.client.events.read().await;
        let mut result = Vec::with_capacity(ids.len());
        for identity in ids {
            match store.get_by_identity(identity) {
                Some(e) => result.push(e.clone()),
                None if ignore_unknown_ids => {}
                None => {
                    return Err(MockError::NotFound(format!(
                        "Event not found: {:?}",
                        identity
                    )))
                }
            }
        }
        Ok(result)
    }

    pub async fn update(&self, patches: &[Patch<PatchEvent>]) -> Result<Vec<Event>> {
        let mut store = self.client.events.write().await;
        let mut result = Vec::with_capacity(patches.len());
        for patch in patches {
            let event = store
                .get_mut_by_identity(&patch.id)
                .ok_or_else(|| MockError::NotFound(format!("Event not found: {:?}", patch.id)))?;
            apply_patch_event(event, &patch.update);
            result.push(event.clone());
        }
        Ok(result)
    }

    pub async fn delete(&self, ids: &[Identity], ignore_unknown_ids: bool) -> Result<()> {
        let mut store = self.client.events.write().await;
        for identity in ids {
            let id = match identity {
                Identity::Id { id } => Some(*id),
                Identity::ExternalId { external_id } => {
                    store.by_ext.get(external_id.as_str()).copied()
                }
            };
            match id {
                Some(id) => {
                    store.remove(id);
                }
                None if ignore_unknown_ids => {}
                None => {
                    return Err(MockError::NotFound(format!(
                        "Event not found: {:?}",
                        identity
                    )))
                }
            }
        }
        Ok(())
    }

    pub async fn filter(&self, req: EventFilterQuery) -> Result<ItemsVec<Event, Cursor>> {
        let store = self.client.events.read().await;
        let filter = req.filter.as_ref();
        let items = store.filter(|e| apply_event_filter(e, filter));
        let limit = req.limit.map(|l| l as usize).unwrap_or(DEFAULT_LIMIT);
        let (page, next_cursor) = paginate(items, req.cursor.as_deref(), limit);
        Ok(ItemsVec {
            items: page,
            extra_fields: Cursor { next_cursor },
        })
    }

    pub async fn filter_all(&self, mut req: EventFilterQuery) -> Result<Vec<Event>> {
        let mut all = Vec::new();
        loop {
            let page = self.filter(req.clone()).await?;
            all.extend(page.items);
            match page.extra_fields.next_cursor {
                Some(cursor) => req.cursor = Some(cursor),
                None => return Ok(all),
            }
        }
    }

    pub async fn search(
        &self,
        filter: EventFilter,
        search: EventSearch,
        limit: Option<u32>,
    ) -> Result<Vec<Event>> {
        let store = self.client.events.read().await;
        let limit = limit.unwrap_or(DEFAULT_LIMIT as u32) as usize;
        let items: Vec<Event> = store
            .filter(|e| apply_event_filter(e, Some(&filter)) && apply_event_search(e, &search))
            .into_iter()
            .take(limit)
            .collect();
        Ok(items)
    }
}

fn apply_event_filter(event: &Event, filter: Option<&EventFilter>) -> bool {
    let Some(filter) = filter else { return true };
    if let Some(source) = &filter.source {
        if event.source.as_ref() != Some(source) {
            return false;
        }
    }
    if let Some(ty) = &filter.r#type {
        if event.r#type.as_ref() != Some(ty) {
            return false;
        }
    }
    if let Some(subtype) = &filter.subtype {
        if event.subtype.as_ref() != Some(subtype) {
            return false;
        }
    }
    if let Some(ext_prefix) = &filter.external_id_prefix {
        if !event
            .external_id
            .as_ref()
            .map(|e| e.starts_with(ext_prefix.as_str()))
            .unwrap_or(false)
        {
            return false;
        }
    }
    if let Some(asset_ids) = &filter.asset_ids {
        if let Some(event_assets) = &event.asset_ids {
            if !asset_ids.iter().any(|id| event_assets.contains(id)) {
                return false;
            }
        } else {
            return false;
        }
    }
    if let Some(ds_ids) = &filter.data_set_ids {
        if let Some(ds_id) = event.data_set_id {
            let matches = ds_ids.iter().any(|id| match id {
                cognite::Identity::Id { id } => *id == ds_id,
                cognite::Identity::ExternalId { .. } => false,
            });
            if !matches {
                return false;
            }
        } else {
            return false;
        }
    }
    if let Some(st) = &filter.start_time {
        if let Some(min) = st.min {
            if event.start_time.map(|t| t < min).unwrap_or(true) {
                return false;
            }
        }
        if let Some(max) = st.max {
            if event.start_time.map(|t| t > max).unwrap_or(true) {
                return false;
            }
        }
    }
    if let Some(metadata) = &filter.metadata {
        if let Some(event_meta) = &event.metadata {
            for (k, v) in metadata {
                if event_meta.get(k).map(|mv| mv != v).unwrap_or(true) {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    true
}

fn apply_event_search(event: &Event, search: &EventSearch) -> bool {
    if let Some(desc) = &search.description {
        if !event
            .description
            .as_ref()
            .map(|d| d.to_lowercase().contains(&desc.to_lowercase()))
            .unwrap_or(false)
        {
            return false;
        }
    }
    true
}

fn apply_patch_event(event: &mut Event, patch: &PatchEvent) {
    event.external_id = apply_set_null(event.external_id.take(), patch.external_id.clone());
    event.data_set_id = apply_set_null(event.data_set_id, patch.data_set_id.clone());
    event.start_time = apply_set_null(event.start_time, patch.start_time.clone());
    event.end_time = apply_set_null(event.end_time, patch.end_time.clone());
    event.description = apply_set_null(event.description.take(), patch.description.clone());
    event.metadata = apply_map(event.metadata.take(), patch.metadata.clone());
    event.asset_ids = apply_list_i64(event.asset_ids.take(), patch.asset_ids.clone());
    event.source = apply_set_null(event.source.take(), patch.source.clone());
    event.r#type = apply_set_null(event.r#type.take(), patch.r#type.clone());
    event.subtype = apply_set_null(event.subtype.take(), patch.subtype.clone());
}
