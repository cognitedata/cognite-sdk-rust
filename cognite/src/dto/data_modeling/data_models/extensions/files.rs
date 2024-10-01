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

use super::{FromReadable, IntoWritable};

#[derive(Clone, Debug, Default)]
/// A special data models instance type.
pub struct CogniteExtractorFile {
    /// The space where the node is located.
    pub space: String,
    /// The external id of the Cognite extractor file.
    pub external_id: String,
    /// Name of the instance.
    pub name: String,
    /// Description of the instance.
    pub description: Option<String>,
    /// Text based labels for generic use, limited to 1000.
    pub tags: Option<Vec<String>>,
    /// Alternative names for the node.
    pub aliases: Option<Vec<String>>,
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

impl From<CogniteExtractorFile> for FileObject {
    fn from(value: CogniteExtractorFile) -> Self {
        Self {
            name: value.name,
            description: value.description,
            tags: value.tags,
            aliases: value.aliases,
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
            name,
            space,
            external_id,
            ..Default::default()
        }
    }
}

impl IntoWritable<FileObject> for CogniteExtractorFile {
    fn try_into_writable(self, view: ViewReference) -> crate::Result<NodeOrEdgeCreate<FileObject>> {
        Ok(NodeOrEdgeCreate::Node(NodeWrite {
            space: self.space.to_owned(),
            external_id: self.external_id.to_owned(),
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
        // TODO: make error better
        match value {
            NodeOrEdge::Node(node_definition) => {
                let mut properties = node_definition
                    .properties
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                let file_object: &FileObject = get_instance_properties(view, &mut properties)
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                Ok(CogniteExtractorFile {
                    external_id: node_definition.external_id,
                    space: node_definition.space,
                    name: file_object.name.clone(),
                    description: file_object.description.clone(),
                    tags: file_object.tags.clone(),
                    aliases: file_object.aliases.clone(),
                    source: file_object.source.clone(),
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
    pub name: String,
    /// Description of the instance.
    pub description: Option<String>,
    /// Text based labels for generic use, limited to 1000.
    pub tags: Option<Vec<String>>,
    /// Alternative names for the node.
    pub aliases: Option<Vec<String>>,
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CogniteSourceable {
    /// Identifier from the source system.
    pub source_id: Option<String>,
    /// Context of the source id. For systems where the sourceId is globally unique, the sourceContext is expected to not be set.
    pub source_context: Option<String>,
    /// Direct relation to a source system.
    pub source: Option<InstanceId>,
    /// When the instance was created in source system (if available).
    pub source_created_time: Option<i64>,
    /// When the instance was last updated in the source system (if available)
    pub source_updated_time: Option<i64>,
    /// User identifier from the source system on who created the source data. This identifier is
    /// not guaranteed to match the user identifiers in CDF.
    pub source_created_user: Option<String>,
    /// User identifier from the source system on who last updated the source data.
    /// This identifier is not guaranteed to match the user identifiers in CDF.
    pub source_updated_user: Option<String>,
}
