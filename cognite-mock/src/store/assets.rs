use cognite::{assets::Asset, Identity};
use std::collections::HashMap;

#[derive(Default)]
pub struct AssetStore {
    pub by_id: HashMap<i64, Asset>,
    pub by_ext: HashMap<String, i64>,
    pub order: Vec<i64>,
}

impl AssetStore {
    pub fn insert(&mut self, asset: Asset) {
        let id = asset.id;
        if let Some(ext) = &asset.external_id {
            self.by_ext.insert(ext.clone(), id);
        }
        if !self.by_id.contains_key(&id) {
            self.order.push(id);
        }
        self.by_id.insert(id, asset);
    }

    pub fn remove(&mut self, id: i64) -> Option<Asset> {
        let asset = self.by_id.remove(&id)?;
        if let Some(ext) = &asset.external_id {
            self.by_ext.remove(ext);
        }
        self.order.retain(|&x| x != id);
        Some(asset)
    }

    pub fn id_for_identity(&self, identity: &Identity) -> Option<i64> {
        match identity {
            Identity::Id { id } => Some(*id),
            Identity::ExternalId { external_id } => self.by_ext.get(external_id).copied(),
        }
    }

    pub fn get_by_identity(&self, identity: &Identity) -> Option<&Asset> {
        match identity {
            Identity::Id { id } => self.by_id.get(id),
            Identity::ExternalId { external_id } => self
                .by_ext
                .get(external_id)
                .and_then(|id| self.by_id.get(id)),
        }
    }

    pub fn get_mut_by_identity(&mut self, identity: &Identity) -> Option<&mut Asset> {
        match identity {
            Identity::Id { id } => self.by_id.get_mut(id),
            Identity::ExternalId { external_id } => {
                let id = *self.by_ext.get(external_id)?;
                self.by_id.get_mut(&id)
            }
        }
    }

    pub fn filter<F: Fn(&Asset) -> bool>(&self, f: F) -> Vec<Asset> {
        self.order
            .iter()
            .filter_map(|id| self.by_id.get(id))
            .filter(|a| f(a))
            .cloned()
            .collect()
    }
}
