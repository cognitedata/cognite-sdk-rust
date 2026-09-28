use cognite::{events::Event, Identity};
use std::collections::HashMap;

#[derive(Default)]
pub struct EventStore {
    pub by_id: HashMap<i64, Event>,
    pub by_ext: HashMap<String, i64>,
    pub order: Vec<i64>,
}

impl EventStore {
    pub fn insert(&mut self, event: Event) {
        let id = event.id;
        if let Some(ext) = &event.external_id {
            self.by_ext.insert(ext.clone(), id);
        }
        if !self.by_id.contains_key(&id) {
            self.order.push(id);
        }
        self.by_id.insert(id, event);
    }

    pub fn remove(&mut self, id: i64) -> Option<Event> {
        let event = self.by_id.remove(&id)?;
        if let Some(ext) = &event.external_id {
            self.by_ext.remove(ext);
        }
        self.order.retain(|&x| x != id);
        Some(event)
    }

    pub fn id_for_identity(&self, identity: &Identity) -> Option<i64> {
        match identity {
            Identity::Id { id } => Some(*id),
            Identity::ExternalId { external_id } => self.by_ext.get(external_id).copied(),
        }
    }

    pub fn get_by_identity(&self, identity: &Identity) -> Option<&Event> {
        match identity {
            Identity::Id { id } => self.by_id.get(id),
            Identity::ExternalId { external_id } => self
                .by_ext
                .get(external_id)
                .and_then(|id| self.by_id.get(id)),
        }
    }

    pub fn get_mut_by_identity(&mut self, identity: &Identity) -> Option<&mut Event> {
        match identity {
            Identity::Id { id } => self.by_id.get_mut(id),
            Identity::ExternalId { external_id } => {
                let id = *self.by_ext.get(external_id)?;
                self.by_id.get_mut(&id)
            }
        }
    }

    pub fn filter<F: Fn(&Event) -> bool>(&self, f: F) -> Vec<Event> {
        self.order
            .iter()
            .filter_map(|id| self.by_id.get(id))
            .filter(|e| f(e))
            .cloned()
            .collect()
    }
}
