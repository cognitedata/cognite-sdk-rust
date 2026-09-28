use std::sync::Arc;

use cognite::time_series::{
    AddTimeSeries, PatchTimeSeries, TimeSeries, TimeSeriesFilter, TimeSeriesFilterRequest,
    TimeSeriesSearch,
};
use cognite::{Cursor, Identity, ItemsVec, Patch};

use crate::client::MockClient;
use crate::error::{MockError, Result};
use crate::patch::{apply_list_i64, apply_map, apply_set_null, apply_set_opt, paginate};

const DEFAULT_LIMIT: usize = 1000;

pub struct MockTimeSeriesResource {
    pub(crate) client: Arc<MockClient>,
}

impl MockTimeSeriesResource {
    pub fn new(client: Arc<MockClient>) -> Self {
        Self { client }
    }

    pub async fn create(&self, creates: &[AddTimeSeries]) -> Result<Vec<TimeSeries>> {
        let mut store = self.client.time_series.write().await;
        let mut result = Vec::with_capacity(creates.len());
        for add in creates {
            if let Some(ext) = &add.external_id {
                if store.by_ext.contains_key(ext.as_str()) {
                    return Err(MockError::AlreadyExists(format!(
                        "TimeSeries with externalId '{}' already exists",
                        ext
                    )));
                }
            }
            let id = self.client.id_gen.next();
            let ts = TimeSeries {
                id,
                external_id: add.external_id.clone(),
                instance_id: None,
                name: add.name.clone(),
                is_string: add.is_string,
                metadata: add.metadata.clone(),
                unit: add.unit.clone(),
                unit_external_id: add.unit_external_id.clone(),
                asset_id: add.asset_id,
                is_step: add.is_step,
                description: add.description.clone(),
                security_categories: add.security_categories.clone(),
                created_time: 0,
                last_updated_time: 0,
                data_set_id: add.data_set_id,
            };
            store.insert(ts.clone());
            result.push(ts);
        }
        Ok(result)
    }

    pub async fn retrieve(
        &self,
        ids: &[Identity],
        ignore_unknown_ids: bool,
    ) -> Result<Vec<TimeSeries>> {
        let store = self.client.time_series.read().await;
        let mut result = Vec::with_capacity(ids.len());
        for identity in ids {
            match store.get_by_identity(identity) {
                Some(ts) => result.push(ts.clone()),
                None if ignore_unknown_ids => {}
                None => {
                    return Err(MockError::NotFound(format!(
                        "TimeSeries not found: {:?}",
                        identity
                    )))
                }
            }
        }
        Ok(result)
    }

    pub async fn update(&self, patches: &[Patch<PatchTimeSeries>]) -> Result<Vec<TimeSeries>> {
        let mut store = self.client.time_series.write().await;
        let mut result = Vec::with_capacity(patches.len());
        for patch in patches {
            let ts = store.get_mut_by_identity(&patch.id).ok_or_else(|| {
                MockError::NotFound(format!("TimeSeries not found: {:?}", patch.id))
            })?;
            apply_patch_ts(ts, &patch.update);
            result.push(ts.clone());
        }
        Ok(result)
    }

    pub async fn delete(&self, ids: &[Identity], ignore_unknown_ids: bool) -> Result<()> {
        let mut store = self.client.time_series.write().await;
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
                        "TimeSeries not found: {:?}",
                        identity
                    )))
                }
            }
        }
        Ok(())
    }

    pub async fn filter(
        &self,
        req: TimeSeriesFilterRequest,
    ) -> Result<ItemsVec<TimeSeries, Cursor>> {
        let store = self.client.time_series.read().await;
        let filter = req.filter.as_ref();
        let items = store.filter(|ts| apply_ts_filter(ts, filter));
        let limit = req.limit.map(|l| l as usize).unwrap_or(DEFAULT_LIMIT);
        let (page, next_cursor) = paginate(items, req.cursor.as_deref(), limit);
        Ok(ItemsVec {
            items: page,
            extra_fields: Cursor { next_cursor },
        })
    }

    pub async fn filter_all(&self, mut req: TimeSeriesFilterRequest) -> Result<Vec<TimeSeries>> {
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
        filter: TimeSeriesFilter,
        search: TimeSeriesSearch,
        limit: Option<u32>,
    ) -> Result<Vec<TimeSeries>> {
        let store = self.client.time_series.read().await;
        let limit = limit.unwrap_or(DEFAULT_LIMIT as u32) as usize;
        let items: Vec<TimeSeries> = store
            .filter(|ts| apply_ts_filter(ts, Some(&filter)) && apply_ts_search(ts, &search))
            .into_iter()
            .take(limit)
            .collect();
        Ok(items)
    }
}

fn apply_ts_filter(ts: &TimeSeries, filter: Option<&TimeSeriesFilter>) -> bool {
    let Some(filter) = filter else { return true };
    if let Some(name) = &filter.name {
        if ts.name.as_ref() != Some(name) {
            return false;
        }
    }
    if let Some(is_string) = filter.is_string {
        if ts.is_string != is_string {
            return false;
        }
    }
    if let Some(is_step) = filter.is_step {
        if ts.is_step != is_step {
            return false;
        }
    }
    if let Some(unit) = &filter.unit {
        if ts.unit.as_ref() != Some(unit) {
            return false;
        }
    }
    if let Some(asset_ids) = &filter.asset_ids {
        if !ts.asset_id.map(|a| asset_ids.contains(&a)).unwrap_or(false) {
            return false;
        }
    }
    if let Some(ext_prefix) = &filter.external_id_prefix {
        if !ts
            .external_id
            .as_ref()
            .map(|e| e.starts_with(ext_prefix.as_str()))
            .unwrap_or(false)
        {
            return false;
        }
    }
    if let Some(ds_ids) = &filter.data_set_ids {
        if let Some(ds_id) = ts.data_set_id {
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
    if let Some(metadata) = &filter.metadata {
        if let Some(ts_meta) = &ts.metadata {
            for (k, v) in metadata {
                if ts_meta.get(k).map(|mv| mv != v).unwrap_or(true) {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    true
}

fn apply_ts_search(ts: &TimeSeries, search: &TimeSeriesSearch) -> bool {
    if let Some(name) = &search.name {
        if !ts
            .name
            .as_ref()
            .map(|n| n.to_lowercase().contains(&name.to_lowercase()))
            .unwrap_or(false)
        {
            return false;
        }
    }
    if let Some(desc) = &search.description {
        if !ts
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

fn apply_patch_ts(ts: &mut TimeSeries, patch: &PatchTimeSeries) {
    ts.name = apply_set_null(ts.name.take(), patch.name.clone());
    ts.external_id = apply_set_null(ts.external_id.take(), patch.external_id.clone());
    ts.metadata = apply_map(ts.metadata.take(), patch.metadata.clone());
    ts.unit = apply_set_null(ts.unit.take(), patch.unit.clone());
    ts.unit_external_id =
        apply_set_null(ts.unit_external_id.take(), patch.unit_external_id.clone());
    ts.asset_id = apply_set_null(ts.asset_id, patch.asset_id.clone());
    ts.description = apply_set_null(ts.description.take(), patch.description.clone());
    ts.security_categories = apply_list_i64(
        ts.security_categories.take(),
        patch.security_categories.clone(),
    );
    ts.data_set_id = apply_set_null(ts.data_set_id, patch.data_set_id.clone());
    if let Some(is_step_patch) = &patch.is_step {
        ts.is_step = is_step_patch.set;
    }
    let _ = apply_set_opt::<bool>; // silence dead code
}
