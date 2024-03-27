use crate::{Identity, IntoParams, LabelsFilter, Range};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Filter on files.
pub struct FileFilter {
    /// Name of files to include.
    pub name: Option<String>,
    /// Filter on this (case-sensitive) prefix for the directory.
    pub directory_prefix: Option<String>,
    /// Mime type of files to include.
    pub mime_type: Option<String>,
    /// Include files belonging to one of these assets.
    pub asset_ids: Option<Vec<u64>>,
    /// Include files belonging to one of these assets.
    pub asset_external_ids: Option<Vec<String>>,
    /// Include files belonging to assets that are in the tree of one of these root assets.
    pub root_asset_ids: Option<Vec<Identity>>,
    /// Include files that belong to one of these data sets.
    pub data_set_ids: Option<Vec<Identity>>,
    /// Include files belonging to assets that are in the subtree of one of these assets.
    pub asset_subtree_ids: Option<Vec<Identity>>,
    /// Source of asset.
    pub source: Option<String>,
    /// Range of timestamps for `created_time`.
    pub created_time: Option<Range<i64>>,
    /// Range of timestamps for `last_updated_time`.
    pub last_updated_time: Option<Range<i64>>,
    /// Range of timestamps for `uploaded_time`.
    pub uploaded_time: Option<Range<i64>>,
    /// Range of timestamps for `source_created_time`.
    pub source_created_time: Option<Range<i64>>,
    /// Range of timestamps for `source_modified_time`.
    pub source_modified_time: Option<Range<i64>>,
    /// Filter by this (case-sensitive) prefix for the external ID.
    pub external_id_prefix: Option<String>,
    /// Filter on files that are uploaded or not uploaded.
    pub uploaded: Option<bool>,
    /// Filter on files matching the given label filter.
    pub labels: Option<LabelsFilter>,
}

impl FileFilter {
    /// Create a new empty file filter.
    pub fn new() -> FileFilter {
        FileFilter::default()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Fuzzy search on files.
pub struct FileSearch {
    /// Fuzzy search on file name.
    pub name: Option<String>,
}

impl FileSearch {
    /// Create an empty file search.
    pub fn new() -> FileSearch {
        FileSearch::default()
    }
}

#[derive(Debug, Default)]
/// Query for file upload requests.
pub struct FileUploadQuery {
    /// Set to `true` to overwrite any files that already exist in CDF.
    pub overwrite: bool,
}

impl IntoParams for FileUploadQuery {
    fn into_params(self) -> Vec<(String, String)> {
        if self.overwrite {
            vec![("overwrite".to_string(), "true".to_string())]
        } else {
            vec![]
        }
    }
}

impl FileUploadQuery {
    /// Create a file upload query
    ///
    /// # Arguments
    ///
    /// * `overwrite` - `true` to overwrite any files that already exist in CDF.
    pub fn new(overwrite: bool) -> Self {
        Self { overwrite }
    }
}
