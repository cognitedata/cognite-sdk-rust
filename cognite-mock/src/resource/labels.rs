use std::sync::Arc;

use cognite::labels::{AddLabel, Label, LabelFilter};
use cognite::{Cursor, ItemsVec};

use crate::client::MockClient;
use crate::error::{MockError, Result};
use crate::patch::paginate;

const DEFAULT_LIMIT: usize = 1000;

pub struct MockLabelsResource {
    pub(crate) client: Arc<MockClient>,
}

impl MockLabelsResource {
    pub fn new(client: Arc<MockClient>) -> Self {
        Self { client }
    }

    pub async fn create(&self, creates: &[AddLabel]) -> Result<Vec<Label>> {
        let mut store = self.client.labels.write().await;
        let mut result = Vec::with_capacity(creates.len());
        for add in creates {
            if store.by_ext.contains_key(add.external_id.as_str()) {
                return Err(MockError::AlreadyExists(format!(
                    "Label with externalId '{}' already exists",
                    add.external_id
                )));
            }
            let label = Label {
                external_id: add.external_id.clone(),
                name: add.name.clone(),
                description: add.description.clone(),
                data_set_id: add.data_set_id,
                created_time: 0,
            };
            store.insert(label.clone());
            result.push(label);
        }
        Ok(result)
    }

    pub async fn list(
        &self,
        filter: Option<LabelFilter>,
        cursor: Option<String>,
        limit: Option<u32>,
    ) -> Result<ItemsVec<Label, Cursor>> {
        let store = self.client.labels.read().await;
        let items = store.filter(|l| apply_label_filter(l, filter.as_ref()));
        let limit = limit.map(|l| l as usize).unwrap_or(DEFAULT_LIMIT);
        let (page, next_cursor) = paginate(items, cursor.as_deref(), limit);
        Ok(ItemsVec {
            items: page,
            extra_fields: Cursor { next_cursor },
        })
    }

    pub async fn delete(&self, external_ids: &[String], ignore_unknown_ids: bool) -> Result<()> {
        let mut store = self.client.labels.write().await;
        for ext_id in external_ids {
            match store.remove(ext_id.as_str()) {
                Some(_) => {}
                None if ignore_unknown_ids => {}
                None => return Err(MockError::NotFound(format!("Label '{}' not found", ext_id))),
            }
        }
        Ok(())
    }
}

fn apply_label_filter(label: &Label, filter: Option<&LabelFilter>) -> bool {
    let Some(filter) = filter else { return true };
    if let Some(name) = &filter.name {
        if &label.name != name {
            return false;
        }
    }
    if let Some(ext_prefix) = &filter.external_id_prefix {
        if !label.external_id.starts_with(ext_prefix.as_str()) {
            return false;
        }
    }
    if let Some(ds_ids) = &filter.data_set_ids {
        if let Some(ds_id) = label.data_set_id {
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
    true
}
