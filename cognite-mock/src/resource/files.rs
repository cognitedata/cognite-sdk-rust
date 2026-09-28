use std::sync::Arc;

use cognite::files::{AddFile, FileFilter, FileMetadata, PatchFile};
use cognite::{Cursor, Identity, ItemsVec, Patch};

use crate::client::MockClient;
use crate::error::{MockError, Result};
use crate::patch::{apply_list_ext_id, apply_list_i64, apply_map, apply_set_null, paginate};

const DEFAULT_LIMIT: usize = 1000;

pub struct MockFilesResource {
    pub(crate) client: Arc<MockClient>,
}

impl MockFilesResource {
    pub fn new(client: Arc<MockClient>) -> Self {
        Self { client }
    }

    /// Create file metadata. The file is created in "not uploaded" state.
    pub async fn create_metadata(&self, creates: &[AddFile]) -> Result<Vec<FileMetadata>> {
        let mut store = self.client.files.write().await;
        let mut result = Vec::with_capacity(creates.len());
        for add in creates {
            if let Some(ext) = &add.external_id {
                if store.by_ext.contains_key(ext.as_str()) {
                    return Err(MockError::AlreadyExists(format!(
                        "File with externalId '{}' already exists",
                        ext
                    )));
                }
            }
            let id = self.client.id_gen.next();
            let meta = FileMetadata {
                id,
                external_id: add.external_id.clone(),
                name: add.name.clone(),
                directory: add.directory.clone(),
                source: add.source.clone(),
                mime_type: add.mime_type.clone(),
                metadata: add.metadata.clone(),
                asset_ids: add.asset_ids.clone(),
                data_set_id: add.data_set_id,
                source_created_time: add.source_created_time,
                source_modified_time: add.source_modified_time,
                security_categories: add.security_categories.clone(),
                labels: add.labels.clone(),
                uploaded: false,
                uploaded_time: None,
                created_time: 0,
                last_updated_time: 0,
            };
            store.insert(meta.clone());
            result.push(meta);
        }
        Ok(result)
    }

    /// Upload bytes for a file, marking it as uploaded.
    pub async fn upload(&self, id: &Identity, data: Vec<u8>) -> Result<()> {
        let mut store = self.client.files.write().await;
        let file_id = store
            .id_for_identity(id)
            .ok_or_else(|| MockError::NotFound(format!("File not found: {:?}", id)))?;
        store.bytes.insert(file_id, data);
        if let Some(meta) = store.by_id.get_mut(&file_id) {
            meta.uploaded = true;
            meta.uploaded_time = Some(0);
        }
        Ok(())
    }

    /// Download bytes for a file.
    pub async fn download(&self, id: &Identity) -> Result<Vec<u8>> {
        let store = self.client.files.read().await;
        let file_id = store
            .id_for_identity(id)
            .ok_or_else(|| MockError::NotFound(format!("File not found: {:?}", id)))?;
        store
            .bytes
            .get(&file_id)
            .cloned()
            .ok_or_else(|| MockError::NotFound(format!("File content not uploaded: {:?}", id)))
    }

    pub async fn retrieve(
        &self,
        ids: &[Identity],
        ignore_unknown_ids: bool,
    ) -> Result<Vec<FileMetadata>> {
        let store = self.client.files.read().await;
        let mut result = Vec::with_capacity(ids.len());
        for identity in ids {
            match store.get_by_identity(identity) {
                Some(m) => result.push(m.clone()),
                None if ignore_unknown_ids => {}
                None => {
                    return Err(MockError::NotFound(format!(
                        "File not found: {:?}",
                        identity
                    )))
                }
            }
        }
        Ok(result)
    }

    pub async fn update(&self, patches: &[Patch<PatchFile>]) -> Result<Vec<FileMetadata>> {
        let mut store = self.client.files.write().await;
        let mut result = Vec::with_capacity(patches.len());
        for patch in patches {
            let meta = store
                .get_mut_by_identity(&patch.id)
                .ok_or_else(|| MockError::NotFound(format!("File not found: {:?}", patch.id)))?;
            apply_patch_file(meta, &patch.update);
            result.push(meta.clone());
        }
        Ok(result)
    }

    pub async fn delete(&self, ids: &[Identity], ignore_unknown_ids: bool) -> Result<()> {
        let mut store = self.client.files.write().await;
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
                        "File not found: {:?}",
                        identity
                    )))
                }
            }
        }
        Ok(())
    }

    pub async fn filter(
        &self,
        filter: Option<FileFilter>,
        cursor: Option<String>,
        limit: Option<u32>,
    ) -> Result<ItemsVec<FileMetadata, Cursor>> {
        let store = self.client.files.read().await;
        let items = store.filter(|m| apply_file_filter(m, filter.as_ref()));
        let limit = limit.map(|l| l as usize).unwrap_or(DEFAULT_LIMIT);
        let (page, next_cursor) = paginate(items, cursor.as_deref(), limit);
        Ok(ItemsVec {
            items: page,
            extra_fields: Cursor { next_cursor },
        })
    }

    pub async fn filter_all(&self, filter: Option<FileFilter>) -> Result<Vec<FileMetadata>> {
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

fn apply_file_filter(meta: &FileMetadata, filter: Option<&FileFilter>) -> bool {
    let Some(filter) = filter else { return true };
    if let Some(name) = &filter.name {
        if meta.name != *name {
            return false;
        }
    }
    if let Some(mime) = &filter.mime_type {
        if meta.mime_type.as_ref() != Some(mime) {
            return false;
        }
    }
    if let Some(uploaded) = filter.uploaded {
        if meta.uploaded != uploaded {
            return false;
        }
    }
    if let Some(ext_prefix) = &filter.external_id_prefix {
        if !meta
            .external_id
            .as_ref()
            .map(|e| e.starts_with(ext_prefix.as_str()))
            .unwrap_or(false)
        {
            return false;
        }
    }
    if let Some(ds_ids) = &filter.data_set_ids {
        if let Some(ds_id) = meta.data_set_id {
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
    if let Some(source) = &filter.source {
        if meta.source.as_ref() != Some(source) {
            return false;
        }
    }
    true
}

fn apply_patch_file(meta: &mut FileMetadata, patch: &PatchFile) {
    meta.external_id = apply_set_null(meta.external_id.take(), patch.external_id.clone());
    meta.directory = apply_set_null(meta.directory.take(), patch.directory.clone());
    meta.source = apply_set_null(meta.source.take(), patch.source.clone());
    meta.mime_type = apply_set_null(meta.mime_type.take(), patch.mime_type.clone());
    meta.metadata = apply_map(meta.metadata.take(), patch.metadata.clone());
    meta.asset_ids = apply_list_i64(meta.asset_ids.take(), patch.asset_ids.clone());
    meta.source_created_time =
        apply_set_null(meta.source_created_time, patch.source_created_time.clone());
    meta.source_modified_time = apply_set_null(
        meta.source_modified_time,
        patch.source_modified_time.clone(),
    );
    meta.security_categories = apply_list_i64(
        meta.security_categories.take(),
        patch.security_categories.clone(),
    );
    meta.labels = apply_list_ext_id(meta.labels.take(), patch.labels.clone());
    meta.data_set_id = apply_set_null(meta.data_set_id, patch.data_set_id.clone());
}
