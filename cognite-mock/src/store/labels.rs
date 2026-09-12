use cognite::labels::Label;
use std::collections::HashMap;

#[derive(Default)]
pub struct LabelStore {
    pub by_ext: HashMap<String, Label>,
    pub order: Vec<String>,
}

impl LabelStore {
    pub fn insert(&mut self, label: Label) {
        let ext = label.external_id.clone();
        if !self.by_ext.contains_key(&ext) {
            self.order.push(ext.clone());
        }
        self.by_ext.insert(ext, label);
    }

    pub fn remove(&mut self, external_id: &str) -> Option<Label> {
        let label = self.by_ext.remove(external_id)?;
        self.order.retain(|x| x != external_id);
        Some(label)
    }

    pub fn filter<F: Fn(&Label) -> bool>(&self, f: F) -> Vec<Label> {
        self.order
            .iter()
            .filter_map(|ext| self.by_ext.get(ext))
            .filter(|l| f(l))
            .cloned()
            .collect()
    }
}
