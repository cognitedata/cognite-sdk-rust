use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::instances::{InstanceId, NodeOrEdgeCreate};

use super::{
    common::{CogniteDescribable, CogniteSourceable},
    CogniteExtendable, WithInstance, WithView,
};

/// A special data models instance type.
pub type CogniteExtractorFile = CogniteExtendable<FileObject>;

impl WithView for CogniteExtractorFile {
    const SPACE: &'static str = "cdf_extraction_extensions";
    const EXTERNAL_ID: &'static str = "CogniteExtractorFile";
    const VERSION: &'static str = "v1";
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// The properties of the file object.
pub struct FileObject {
    /// Name of the instance.
    #[serde(flatten)]
    pub description: CogniteDescribable,
    #[serde(flatten)]
    /// Source system.
    pub source: CogniteSourceable,
    /// List of assets to which this file relates.
    pub assets: Option<Vec<InstanceId>>,
    /// MIME type of the file.
    pub mime_type: Option<String>,
    /// Contains the path elements from the source (for when the source system has a file system
    /// hierarchy or similar).
    pub directory: Option<String>,
    /// Whether the file content has been uploaded to Cognite Data Fusion.
    pub is_uploaded: Option<bool>,
    /// Point in time when the file upload was completed and the file was made available.
    pub uploaded_time: Option<i64>,
    /// Direct relation to an instance of CogniteFileCategory representing the detected
    /// categorization/class for the file.
    pub category: Option<InstanceId>,
    /// Unstructured information extracted from source system.
    pub extracted_data: Option<HashMap<String, String>>,
}

impl FileObject {
    /// Create a new file object.
    pub fn new(name: String) -> FileObject {
        Self {
            description: CogniteDescribable::new(name),
            ..Default::default()
        }
    }
}

impl From<CogniteExtractorFile> for NodeOrEdgeCreate<FileObject> {
    fn from(value: CogniteExtractorFile) -> NodeOrEdgeCreate<FileObject> {
        value.instance()
    }
}
