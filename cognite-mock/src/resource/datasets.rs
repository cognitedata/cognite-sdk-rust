use std::sync::Arc;

use cognite::datasets::{AddDataSet, DataSet, DataSetFilter, PatchDataSet};
use cognite::{Cursor, Identity, ItemsVec, Patch};

use crate::client::MockClient;
use crate::error::{MockError, Result};
use crate::patch::{apply_map, apply_set_null, paginate};

const DEFAULT_LIMIT: usize = 1000;

pub struct MockDataSetsResource {
    pub(crate) client: Arc<MockClient>,
}

impl MockDataSetsResource {
    pub fn new(client: Arc<MockClient>) -> Self {
        Self { client }
    }

    pub async fn create(&self, creates: &[AddDataSet]) -> Result<Vec<DataSet>> {
        let mut store = self.client.data_sets.write().await;
        let mut result = Vec::with_capacity(creates.len());
        for add in creates {
            if let Some(ext) = &add.external_id {
                if store.by_ext.contains_key(ext.as_str()) {
                    return Err(MockError::AlreadyExists(format!(
                        "DataSet with externalId '{}' already exists",
                        ext
                    )));
                }
            }
            let id = self.client.id_gen.next();
            let ds = DataSet {
                id,
                external_id: add.external_id.clone(),
                name: add.name.clone(),
                description: add.description.clone(),
                metadata: add.metadata.clone(),
                write_protected: add.write_protected,
                created_time: 0,
                last_updated_time: 0,
            };
            let ret: DataSet = serde_json::from_value(serde_json::to_value(&ds).unwrap()).unwrap();
            store.insert(ds);
            result.push(ret);
        }
        Ok(result)
    }

    pub async fn retrieve(
        &self,
        ids: &[Identity],
        ignore_unknown_ids: bool,
    ) -> Result<Vec<DataSet>> {
        let store = self.client.data_sets.read().await;
        let mut result = Vec::with_capacity(ids.len());
        for identity in ids {
            let ds = match identity {
                Identity::Id { id } => store.get_by_id(*id),
                Identity::ExternalId { external_id } => store.get_by_ext(external_id.as_str()),
            };
            match ds {
                Some(d) => {
                    result.push(serde_json::from_value(serde_json::to_value(d).unwrap()).unwrap());
                }
                None if ignore_unknown_ids => {}
                None => {
                    return Err(MockError::NotFound(format!(
                        "DataSet not found: {:?}",
                        identity
                    )))
                }
            }
        }
        Ok(result)
    }

    pub async fn update(&self, patches: &[Patch<PatchDataSet>]) -> Result<Vec<DataSet>> {
        let mut store = self.client.data_sets.write().await;
        let mut result = Vec::with_capacity(patches.len());
        for patch in patches {
            let ds = match &patch.id {
                Identity::Id { id } => store.get_mut_by_id(*id),
                Identity::ExternalId { external_id } => store.get_mut_by_ext(external_id.as_str()),
            }
            .ok_or_else(|| MockError::NotFound(format!("DataSet not found: {:?}", patch.id)))?;
            apply_patch_dataset(ds, &patch.update);
            let ret: DataSet = serde_json::from_value(serde_json::to_value(&*ds).unwrap()).unwrap();
            result.push(ret);
        }
        Ok(result)
    }

    pub async fn delete(&self, ids: &[Identity], ignore_unknown_ids: bool) -> Result<()> {
        let mut store = self.client.data_sets.write().await;
        for identity in ids {
            let id = match identity {
                Identity::Id { id } => Some(*id),
                Identity::ExternalId { external_id } => {
                    store.id_for_external_id(external_id.as_str())
                }
            };
            match id {
                Some(id) => {
                    store.remove(id);
                }
                None if ignore_unknown_ids => {}
                None => {
                    return Err(MockError::NotFound(format!(
                        "DataSet not found: {:?}",
                        identity
                    )))
                }
            }
        }
        Ok(())
    }

    pub async fn filter(
        &self,
        filter: Option<DataSetFilter>,
        cursor: Option<String>,
        limit: Option<u32>,
    ) -> Result<ItemsVec<DataSet, Cursor>> {
        let store = self.client.data_sets.read().await;
        let items = store.filter(|d| apply_dataset_filter(d, filter.as_ref()));
        let limit = limit.map(|l| l as usize).unwrap_or(DEFAULT_LIMIT);
        let (page, next_cursor) = paginate(items, cursor.as_deref(), limit);
        Ok(ItemsVec {
            items: page,
            extra_fields: Cursor { next_cursor },
        })
    }

    pub async fn filter_all(&self, filter: Option<DataSetFilter>) -> Result<Vec<DataSet>> {
        let mut all = Vec::new();
        let mut cursor = None;
        loop {
            let page = self.filter(filter.clone(), cursor, None).await?;
            all.extend(page.items);
            match page.extra_fields.next_cursor {
                Some(c) => cursor = Some(c),
                None => return Ok(all),
            }
        }
    }
}

fn apply_dataset_filter(ds: &DataSet, filter: Option<&DataSetFilter>) -> bool {
    let Some(filter) = filter else { return true };
    if let Some(wp) = filter.write_protected {
        if ds.write_protected != wp {
            return false;
        }
    }
    if let Some(ext_prefix) = &filter.external_id_prefix {
        if !ds
            .external_id
            .as_ref()
            .map(|e| e.starts_with(ext_prefix.as_str()))
            .unwrap_or(false)
        {
            return false;
        }
    }
    if let Some(metadata) = &filter.metadata {
        if let Some(ds_meta) = &ds.metadata {
            for (k, v) in metadata {
                if ds_meta.get(k).map(|mv| mv != v).unwrap_or(true) {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    if let Some(ct) = &filter.created_time {
        if let Some(min) = ct.min {
            if ds.created_time < min {
                return false;
            }
        }
        if let Some(max) = ct.max {
            if ds.created_time > max {
                return false;
            }
        }
    }
    if let Some(lut) = &filter.last_updated_time {
        if let Some(min) = lut.min {
            if ds.last_updated_time < min {
                return false;
            }
        }
        if let Some(max) = lut.max {
            if ds.last_updated_time > max {
                return false;
            }
        }
    }
    true
}

fn apply_patch_dataset(ds: &mut DataSet, patch: &PatchDataSet) {
    ds.external_id = apply_set_null(ds.external_id.take(), patch.external_id.clone());
    ds.name = apply_set_null(ds.name.take(), patch.name.clone());
    ds.description = apply_set_null(ds.description.take(), patch.description.clone());
    ds.metadata = apply_map(ds.metadata.take(), patch.metadata.clone());
}
