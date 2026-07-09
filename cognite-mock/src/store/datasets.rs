use cognite::datasets::DataSet;
use std::collections::HashMap;

#[derive(Default)]
pub struct DataSetStore {
    pub by_id: HashMap<i64, DataSet>,
    pub by_ext: HashMap<String, i64>,
    pub order: Vec<i64>,
}

impl DataSetStore {
    pub fn insert(&mut self, ds: DataSet) {
        let id = ds.id;
        if let Some(ext) = &ds.external_id {
            self.by_ext.insert(ext.clone(), id);
        }
        if !self.by_id.contains_key(&id) {
            self.order.push(id);
        }
        self.by_id.insert(id, ds);
    }

    pub fn remove(&mut self, id: i64) -> Option<DataSet> {
        let ds = self.by_id.remove(&id)?;
        if let Some(ext) = &ds.external_id {
            self.by_ext.remove(ext);
        }
        self.order.retain(|&x| x != id);
        Some(ds)
    }

    pub fn id_for_external_id(&self, ext: &str) -> Option<i64> {
        self.by_ext.get(ext).copied()
    }

    pub fn get_by_id(&self, id: i64) -> Option<&DataSet> {
        self.by_id.get(&id)
    }

    pub fn get_by_ext(&self, ext: &str) -> Option<&DataSet> {
        self.by_ext.get(ext).and_then(|id| self.by_id.get(id))
    }

    pub fn get_mut_by_id(&mut self, id: i64) -> Option<&mut DataSet> {
        self.by_id.get_mut(&id)
    }

    pub fn get_mut_by_ext(&mut self, ext: &str) -> Option<&mut DataSet> {
        let id = *self.by_ext.get(ext)?;
        self.by_id.get_mut(&id)
    }

    pub fn filter<F: Fn(&DataSet) -> bool>(&self, f: F) -> Vec<DataSet> {
        self.order
            .iter()
            .filter_map(|id| self.by_id.get(id))
            .filter(|d| f(d))
            .map(|d| serde_json::from_value(serde_json::to_value(d).unwrap()).unwrap())
            .collect()
    }
}
