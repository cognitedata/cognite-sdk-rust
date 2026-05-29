use cognite::relationships::Relationship;
use std::collections::HashMap;

#[derive(Default)]
pub struct RelationshipStore {
    pub by_ext: HashMap<String, Relationship>,
    pub order: Vec<String>,
}

impl RelationshipStore {
    pub fn insert(&mut self, rel: Relationship) {
        let ext = rel.external_id.clone();
        if !self.by_ext.contains_key(&ext) {
            self.order.push(ext.clone());
        }
        self.by_ext.insert(ext, rel);
    }

    pub fn remove(&mut self, external_id: &str) -> Option<Relationship> {
        let rel = self.by_ext.remove(external_id)?;
        self.order.retain(|x| x != external_id);
        Some(rel)
    }

    pub fn get(&self, external_id: &str) -> Option<&Relationship> {
        self.by_ext.get(external_id)
    }

    pub fn get_mut(&mut self, external_id: &str) -> Option<&mut Relationship> {
        self.by_ext.get_mut(external_id)
    }

    pub fn filter<F: Fn(&Relationship) -> bool>(&self, f: F) -> Vec<Relationship> {
        self.order
            .iter()
            .filter_map(|ext| self.by_ext.get(ext))
            .filter(|r| f(r))
            .map(|r| serde_json::from_value(serde_json::to_value(r).unwrap()).unwrap())
            .collect()
    }
}
