use std::sync::Arc;

use cognite::relationships::{AddRelationship, PatchRelationship, Relationship};
use cognite::{Cursor, ItemsVec, Patch};

use crate::client::MockClient;
use crate::error::{MockError, Result};
use crate::patch::{apply_list_ext_id, apply_set, apply_set_null, paginate};

const DEFAULT_LIMIT: usize = 1000;

pub struct MockRelationshipsResource {
    pub(crate) client: Arc<MockClient>,
}

impl MockRelationshipsResource {
    pub fn new(client: Arc<MockClient>) -> Self {
        Self { client }
    }

    pub async fn create(&self, creates: &[AddRelationship]) -> Result<Vec<Relationship>> {
        let mut store = self.client.relationships.write().await;
        let mut result = Vec::with_capacity(creates.len());
        for add in creates {
            if store.by_ext.contains_key(add.external_id.as_str()) {
                return Err(MockError::AlreadyExists(format!(
                    "Relationship with externalId '{}' already exists",
                    add.external_id
                )));
            }
            let rel = Relationship {
                external_id: add.external_id.clone(),
                source_external_id: add.source_external_id.clone(),
                source_type: add.source_type,
                target_external_id: add.target_external_id.clone(),
                target_type: add.target_type,
                start_time: add.start_time,
                end_time: add.end_time,
                confidence: add.confidence,
                data_set_id: add.data_set_id,
                labels: add.labels.clone(),
                created_time: None,
                last_updated_time: None,
                source: None,
                target: None,
            };
            let ret: Relationship =
                serde_json::from_value(serde_json::to_value(&rel).unwrap()).unwrap();
            store.insert(rel);
            result.push(ret);
        }
        Ok(result)
    }

    pub async fn retrieve(
        &self,
        external_ids: &[String],
        ignore_unknown_ids: bool,
    ) -> Result<Vec<Relationship>> {
        let store = self.client.relationships.read().await;
        let mut result = Vec::with_capacity(external_ids.len());
        for ext_id in external_ids {
            match store.get(ext_id.as_str()) {
                Some(r) => {
                    result.push(serde_json::from_value(serde_json::to_value(r).unwrap()).unwrap());
                }
                None if ignore_unknown_ids => {}
                None => {
                    return Err(MockError::NotFound(format!(
                        "Relationship '{}' not found",
                        ext_id
                    )))
                }
            }
        }
        Ok(result)
    }

    pub async fn update(&self, patches: &[Patch<PatchRelationship>]) -> Result<Vec<Relationship>> {
        let mut store = self.client.relationships.write().await;
        let mut result = Vec::with_capacity(patches.len());
        for patch in patches {
            let ext_id = patch.id.as_external_id().ok_or_else(|| {
                MockError::InvalidOperation(
                    "Relationships can only be updated by externalId".to_string(),
                )
            })?;
            let rel = store.get_mut(ext_id.as_str()).ok_or_else(|| {
                MockError::NotFound(format!("Relationship '{}' not found", ext_id))
            })?;
            apply_patch_relationship(rel, &patch.update);
            let ret: Relationship =
                serde_json::from_value(serde_json::to_value(&*rel).unwrap()).unwrap();
            result.push(ret);
        }
        Ok(result)
    }

    pub async fn delete(&self, external_ids: &[String], ignore_unknown_ids: bool) -> Result<()> {
        let mut store = self.client.relationships.write().await;
        for ext_id in external_ids {
            match store.remove(ext_id.as_str()) {
                Some(_) => {}
                None if ignore_unknown_ids => {}
                None => {
                    return Err(MockError::NotFound(format!(
                        "Relationship '{}' not found",
                        ext_id
                    )))
                }
            }
        }
        Ok(())
    }

    pub async fn filter(
        &self,
        filter: Option<RelationshipFilter>,
        cursor: Option<String>,
        limit: Option<u32>,
    ) -> Result<ItemsVec<Relationship, Cursor>> {
        let store = self.client.relationships.read().await;
        let items = store.filter(|r| apply_relationship_filter(r, filter.as_ref()));
        let limit = limit.map(|l| l as usize).unwrap_or(DEFAULT_LIMIT);
        let (page, next_cursor) = paginate(items, cursor.as_deref(), limit);
        Ok(ItemsVec {
            items: page,
            extra_fields: Cursor { next_cursor },
        })
    }

    pub async fn filter_all(
        &self,
        filter: Option<RelationshipFilter>,
    ) -> Result<Vec<Relationship>> {
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

/// Simple filter for relationships (subset of the full CDF filter).
#[derive(Default, Clone)]
pub struct RelationshipFilter {
    pub source_external_ids: Option<Vec<String>>,
    pub target_external_ids: Option<Vec<String>>,
    pub data_set_ids: Option<Vec<i64>>,
    pub external_id_prefix: Option<String>,
}

fn apply_relationship_filter(rel: &Relationship, filter: Option<&RelationshipFilter>) -> bool {
    let Some(filter) = filter else { return true };
    if let Some(srcs) = &filter.source_external_ids {
        if !srcs.contains(&rel.source_external_id) {
            return false;
        }
    }
    if let Some(tgts) = &filter.target_external_ids {
        if !tgts.contains(&rel.target_external_id) {
            return false;
        }
    }
    if let Some(ds_ids) = &filter.data_set_ids {
        if !rel
            .data_set_id
            .map(|id| ds_ids.contains(&id))
            .unwrap_or(false)
        {
            return false;
        }
    }
    if let Some(ext_prefix) = &filter.external_id_prefix {
        if !rel.external_id.starts_with(ext_prefix.as_str()) {
            return false;
        }
    }
    true
}

fn apply_patch_relationship(rel: &mut Relationship, patch: &PatchRelationship) {
    rel.source_type = apply_set(rel.source_type, patch.source_type.clone());
    rel.source_external_id = apply_set(
        rel.source_external_id.clone(),
        patch.source_external_id.clone(),
    );
    rel.target_type = apply_set(rel.target_type, patch.target_type.clone());
    rel.target_external_id = apply_set(
        rel.target_external_id.clone(),
        patch.target_external_id.clone(),
    );
    rel.confidence = apply_set_null(rel.confidence, patch.confidence.clone());
    rel.start_time = apply_set_null(rel.start_time, patch.start_time.clone());
    rel.end_time = apply_set_null(rel.end_time, patch.end_time.clone());
    rel.data_set_id = apply_set_null(rel.data_set_id, patch.data_set_id.clone());
    rel.labels = apply_list_ext_id(rel.labels.take(), patch.labels.clone());
}
