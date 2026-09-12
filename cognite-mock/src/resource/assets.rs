use std::sync::Arc;

use cognite::assets::{AddAsset, Asset, AssetFilter, AssetSearch, FilterAssetsRequest, PatchAsset};
use cognite::{Cursor, Identity, ItemsVec, Patch};

use crate::client::MockClient;
use crate::error::{MockError, Result};
use crate::patch::{apply_list_ext_id, apply_map, apply_set_null, apply_set_opt, paginate};

const DEFAULT_LIMIT: usize = 1000;

pub struct MockAssetsResource {
    pub(crate) client: Arc<MockClient>,
}

impl MockAssetsResource {
    pub fn new(client: Arc<MockClient>) -> Self {
        Self { client }
    }

    pub async fn create(&self, creates: &[AddAsset]) -> Result<Vec<Asset>> {
        let mut store = self.client.assets.write().await;
        let mut result = Vec::with_capacity(creates.len());
        for add in creates {
            if let Some(ext) = &add.external_id {
                if store.by_ext.contains_key(ext.as_str()) {
                    return Err(MockError::AlreadyExists(format!(
                        "Asset with externalId '{}' already exists",
                        ext
                    )));
                }
            }
            let id = self.client.id_gen.next();
            let asset = Asset {
                id,
                name: add.name.clone(),
                external_id: add.external_id.clone(),
                parent_id: add.parent_id,
                parent_external_id: add.parent_external_id.clone(),
                description: add.description.clone(),
                metadata: add.metadata.clone(),
                source: add.source.clone(),
                created_time: 0,
                last_updated_time: 0,
                root_id: None,
                aggregates: None,
                data_set_id: add.data_set_id,
                labels: add.labels.clone(),
                geo_location: add.geo_location.clone(),
            };
            store.insert(asset.clone());
            result.push(asset);
        }
        Ok(result)
    }

    pub async fn retrieve(&self, ids: &[Identity], ignore_unknown_ids: bool) -> Result<Vec<Asset>> {
        let store = self.client.assets.read().await;
        let mut result = Vec::with_capacity(ids.len());
        for identity in ids {
            match store.get_by_identity(identity) {
                Some(a) => result.push(a.clone()),
                None if ignore_unknown_ids => {}
                None => {
                    return Err(MockError::NotFound(format!(
                        "Asset not found: {:?}",
                        identity
                    )))
                }
            }
        }
        Ok(result)
    }

    pub async fn update(&self, patches: &[Patch<PatchAsset>]) -> Result<Vec<Asset>> {
        let mut store = self.client.assets.write().await;
        let mut result = Vec::with_capacity(patches.len());
        for patch in patches {
            let asset = store
                .get_mut_by_identity(&patch.id)
                .ok_or_else(|| MockError::NotFound(format!("Asset not found: {:?}", patch.id)))?;
            apply_patch_asset(asset, &patch.update);
            asset.last_updated_time = 0;
            result.push(asset.clone());
        }
        Ok(result)
    }

    pub async fn delete(&self, ids: &[Identity], ignore_unknown_ids: bool) -> Result<()> {
        let mut store = self.client.assets.write().await;
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
                        "Asset not found: {:?}",
                        identity
                    )))
                }
            }
        }
        Ok(())
    }

    pub async fn filter(&self, req: FilterAssetsRequest) -> Result<ItemsVec<Asset, Cursor>> {
        let store = self.client.assets.read().await;
        let filter = req.filter.as_ref();
        let items = store.filter(|a| apply_asset_filter(a, filter));
        let limit = req.limit.map(|l| l as usize).unwrap_or(DEFAULT_LIMIT);
        let (page, next_cursor) = paginate(items, req.cursor.as_deref(), limit);
        Ok(ItemsVec {
            items: page,
            extra_fields: Cursor { next_cursor },
        })
    }

    pub async fn filter_all(&self, mut req: FilterAssetsRequest) -> Result<Vec<Asset>> {
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
        filter: AssetFilter,
        search: AssetSearch,
        limit: Option<u32>,
    ) -> Result<Vec<Asset>> {
        let store = self.client.assets.read().await;
        let limit = limit.unwrap_or(DEFAULT_LIMIT as u32) as usize;
        let items: Vec<Asset> = store
            .filter(|a| apply_asset_filter(a, Some(&filter)) && apply_asset_search(a, &search))
            .into_iter()
            .take(limit)
            .collect();
        Ok(items)
    }
}

fn apply_asset_filter(asset: &Asset, filter: Option<&AssetFilter>) -> bool {
    let Some(filter) = filter else { return true };
    if let Some(name) = &filter.name {
        if &asset.name != name {
            return false;
        }
    }
    if let Some(parent_ids) = &filter.parent_ids {
        if !asset
            .parent_id
            .map(|p| parent_ids.contains(&p))
            .unwrap_or(false)
        {
            return false;
        }
    }
    if let Some(parent_ext_ids) = &filter.parent_external_ids {
        if !asset
            .parent_external_id
            .as_ref()
            .map(|p| parent_ext_ids.contains(p))
            .unwrap_or(false)
        {
            return false;
        }
    }
    if let Some(source) = &filter.source {
        if asset.source.as_ref() != Some(source) {
            return false;
        }
    }
    if let Some(ext_prefix) = &filter.external_id_prefix {
        if !asset
            .external_id
            .as_ref()
            .map(|e| e.starts_with(ext_prefix.as_str()))
            .unwrap_or(false)
        {
            return false;
        }
    }
    if let Some(data_set_ids) = &filter.data_set_ids {
        if let Some(ds_id) = asset.data_set_id {
            let matches = data_set_ids.iter().any(|id| match id {
                Identity::Id { id } => *id == ds_id,
                Identity::ExternalId { .. } => false,
            });
            if !matches {
                return false;
            }
        } else {
            return false;
        }
    }
    if let Some(metadata) = &filter.metadata {
        if let Some(asset_meta) = &asset.metadata {
            for (k, v) in metadata {
                if asset_meta.get(k).map(|av| av != v).unwrap_or(true) {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    if let Some(ct) = &filter.created_time {
        if let Some(min) = ct.min {
            if asset.created_time < min {
                return false;
            }
        }
        if let Some(max) = ct.max {
            if asset.created_time > max {
                return false;
            }
        }
    }
    if let Some(lut) = &filter.last_updated_time {
        if let Some(min) = lut.min {
            if asset.last_updated_time < min {
                return false;
            }
        }
        if let Some(max) = lut.max {
            if asset.last_updated_time > max {
                return false;
            }
        }
    }
    true
}

fn apply_asset_search(asset: &Asset, search: &AssetSearch) -> bool {
    if let Some(name) = &search.name {
        if !asset.name.to_lowercase().contains(&name.to_lowercase()) {
            return false;
        }
    }
    if let Some(desc) = &search.description {
        if !asset
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

fn apply_patch_asset(asset: &mut Asset, patch: &PatchAsset) {
    asset.external_id = apply_set_null(asset.external_id.take(), patch.external_id.clone());
    if let Some(name_patch) = &patch.name {
        asset.name = name_patch.set.clone();
    }
    asset.description = apply_set_null(asset.description.take(), patch.description.clone());
    asset.data_set_id = apply_set_null(asset.data_set_id, patch.data_set_id.clone());
    asset.metadata = apply_map(asset.metadata.take(), patch.metadata.clone());
    asset.source = apply_set_null(asset.source.take(), patch.source.clone());
    asset.parent_id = apply_set_opt(asset.parent_id, patch.parent_id.clone());
    asset.parent_external_id = apply_set_opt(
        asset.parent_external_id.take(),
        patch.parent_external_id.clone(),
    );
    asset.labels = apply_list_ext_id(asset.labels.take(), patch.labels.clone());
    asset.geo_location = apply_set_null(asset.geo_location.take(), patch.geo_location.clone());
}
