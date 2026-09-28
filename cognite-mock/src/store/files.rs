use cognite::{files::FileMetadata, Identity};
use std::collections::HashMap;

#[derive(Default)]
pub struct FileMetaStore {
    pub by_id: HashMap<i64, FileMetadata>,
    pub by_ext: HashMap<String, i64>,
    pub order: Vec<i64>,
    pub bytes: HashMap<i64, Vec<u8>>,
}

impl FileMetaStore {
    pub fn insert(&mut self, meta: FileMetadata) {
        let id = meta.id;
        if let Some(ext) = &meta.external_id {
            self.by_ext.insert(ext.clone(), id);
        }
        if !self.by_id.contains_key(&id) {
            self.order.push(id);
        }
        self.by_id.insert(id, meta);
    }

    pub fn remove(&mut self, id: i64) -> Option<FileMetadata> {
        let meta = self.by_id.remove(&id)?;
        if let Some(ext) = &meta.external_id {
            self.by_ext.remove(ext);
        }
        self.order.retain(|&x| x != id);
        self.bytes.remove(&id);
        Some(meta)
    }

    pub fn id_for_identity(&self, identity: &Identity) -> Option<i64> {
        match identity {
            Identity::Id { id } => Some(*id),
            Identity::ExternalId { external_id } => self.by_ext.get(external_id).copied(),
        }
    }

    pub fn get_by_identity(&self, identity: &Identity) -> Option<&FileMetadata> {
        match identity {
            Identity::Id { id } => self.by_id.get(id),
            Identity::ExternalId { external_id } => self
                .by_ext
                .get(external_id)
                .and_then(|id| self.by_id.get(id)),
        }
    }

    pub fn get_mut_by_identity(&mut self, identity: &Identity) -> Option<&mut FileMetadata> {
        match identity {
            Identity::Id { id } => self.by_id.get_mut(id),
            Identity::ExternalId { external_id } => {
                let id = *self.by_ext.get(external_id)?;
                self.by_id.get_mut(&id)
            }
        }
    }

    pub fn filter<F: Fn(&FileMetadata) -> bool>(&self, f: F) -> Vec<FileMetadata> {
        self.order
            .iter()
            .filter_map(|id| self.by_id.get(id))
            .filter(|m| f(m))
            .cloned()
            .collect()
    }
}
