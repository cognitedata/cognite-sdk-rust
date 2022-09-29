use crate::Range;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded: Option<bool>,
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
