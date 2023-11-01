use crate::{AsParams, Identity, LabelsFilter, Range};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileFilter {
    pub name: Option<String>,
    pub directory_prefix: Option<String>,
    pub mime_type: Option<String>,
    pub asset_ids: Option<Vec<u64>>,
    pub asset_external_ids: Option<Vec<String>>,
    pub root_asset_ids: Option<Vec<Identity>>,
    pub data_set_ids: Option<Vec<Identity>>,
    pub asset_subtree_ids: Option<Vec<Identity>>,
    pub source: Option<String>,
    pub created_time: Option<Range<i64>>,
    pub last_updated_time: Option<Range<i64>>,
    pub uploaded_time: Option<Range<i64>>,
    pub source_created_time: Option<Range<i64>>,
    pub source_modified_time: Option<Range<i64>>,
    pub external_id_prefix: Option<String>,
    pub uploaded: Option<bool>,
    pub labels: Option<LabelsFilter>,
}

impl FileFilter {
    pub fn new() -> FileFilter {
        FileFilter::default()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileSearch {
    pub name: Option<String>,
}

impl FileSearch {
    pub fn new() -> FileSearch {
        FileSearch::default()
    }
}

#[derive(Debug, Default)]
pub struct FileUploadQuery {
    pub overwrite: bool,
}

impl AsParams for FileUploadQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        if self.overwrite {
            vec![("overwrite".to_string(), "true".to_string())]
        } else {
            vec![]
        }
    }
}

impl FileUploadQuery {
    pub fn new(overwrite: bool) -> Self {
        Self { overwrite }
    }
}
