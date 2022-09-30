use crate::{AsParams, Identity, LabelsFilter, Range};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_external_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_asset_ids: Option<Vec<Identity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_ids: Option<Vec<Identity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_subtree_ids: Option<Vec<Identity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_created_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_modified_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<LabelsFilter>,
}

impl FileFilter {
    pub fn new() -> FileFilter {
        FileFilter::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
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
