use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    get_instance_properties,
    models::{
        instances::{EdgeOrNodeData, InstanceId, NodeOrEdge, NodeOrEdgeCreate, NodeWrite},
        views::ViewReference,
        SourceReference,
    },
    Error,
};

use super::{
    common::{CogniteAuditable, CogniteDescribable, CogniteSourceable},
    FromReadable, IntoWritable,
};

#[derive(Clone, Debug, Default)]
/// A special data models instance type.
pub struct CogniteExtractorFile {
    /// Id of the instance.
    pub id: InstanceId,
    /// Cognite describable.
    pub description: CogniteDescribable,
    /// Cognite sourceable.
    pub source: CogniteSourceable,
    /// An audit of the lifecycle of the instance
    pub audit: CogniteAuditable,
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

impl From<CogniteExtractorFile> for FileObject {
    fn from(value: CogniteExtractorFile) -> Self {
        Self {
            description: value.description,
            source: value.source,
            assets: value.assets,
            mime_type: value.mime_type,
            directory: value.directory,
            is_uploaded: value.is_uploaded,
            uploaded_time: value.uploaded_time,
            category: value.category,
            extracted_data: value.extracted_data,
        }
    }
}

impl CogniteExtractorFile {
    /// Create a new instance of this type.
    ///
    /// # Arguments
    ///
    /// * `space` - The space where this entity will be saved.
    /// * `external_id` - A unique external id for this entity.
    /// * `name` - A name for the entity.
    pub fn new(space: String, external_id: String, name: String) -> Self {
        CogniteExtractorFile {
            description: CogniteDescribable {
                name,
                ..Default::default()
            },
            id: InstanceId { space, external_id },
            ..Default::default()
        }
    }
}

impl IntoWritable<FileObject> for CogniteExtractorFile {
    fn try_into_writable(self, view: ViewReference) -> crate::Result<NodeOrEdgeCreate<FileObject>> {
        Ok(NodeOrEdgeCreate::Node(NodeWrite {
            space: self.id.space.to_owned(),
            external_id: self.id.external_id.to_owned(),
            existing_version: None,
            r#type: None,
            sources: Some(vec![EdgeOrNodeData {
                source: SourceReference::View(view),
                properties: self.into(),
            }]),
        }))
    }
}

impl FromReadable<FileObject> for CogniteExtractorFile {
    fn try_from_readable(
        value: NodeOrEdge<FileObject>,
        view: ViewReference,
    ) -> crate::Result<CogniteExtractorFile> {
        match value {
            NodeOrEdge::Node(node_definition) => {
                let mut properties = node_definition
                    .properties
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                let file_object: &FileObject = get_instance_properties(view, &mut properties)
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                Ok(CogniteExtractorFile {
                    id: InstanceId {
                        external_id: node_definition.external_id,
                        space: node_definition.space,
                    },
                    description: file_object.description.clone(),
                    source: file_object.source.clone(),
                    audit: CogniteAuditable {
                        created_time: node_definition.created_time,
                        last_updated_time: node_definition.last_updated_time,
                        deleted_time: node_definition.deleted_time,
                    },
                    assets: file_object.assets.clone(),
                    mime_type: file_object.mime_type.clone(),
                    directory: file_object.directory.clone(),
                    is_uploaded: file_object.is_uploaded,
                    uploaded_time: file_object.uploaded_time,
                    category: file_object.category.clone(),
                    extracted_data: file_object.extracted_data.clone(),
                })
            }
            _ => Err(Error::Other("Invalid type".to_string())),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
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
