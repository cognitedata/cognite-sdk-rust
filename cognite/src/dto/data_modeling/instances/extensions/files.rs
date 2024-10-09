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
    FromReadable, WithInstance, WithView,
};

#[derive(Clone, Debug, Default)]
/// A special data models instance type.
pub struct CogniteExtractorFile {
    /// The where the instance belong. This can be none if the default view is preferred.
    pub view: Option<ViewReference>,
    /// Id of the instance.
    pub id: InstanceId,
    /// An audit of the lifecycle of the instance
    pub audit: CogniteAuditable,
    /// File object.
    pub file_object: FileObject,
}

impl WithView for CogniteExtractorFile {
    const SPACE: &'static str = "cdf_extraction_extensions";
    const EXTERNAL_ID: &'static str = "CogniteExtractorFile";
    const VERSION: &'static str = "v1";
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
            id: InstanceId { space, external_id },
            view: None,
            file_object: FileObject::new(name),
            ..Default::default()
        }
    }
}

impl WithInstance<FileObject> for CogniteExtractorFile {
    fn instance(self) -> NodeOrEdgeCreate<FileObject> {
        NodeOrEdgeCreate::Node(NodeWrite {
            space: self.id.space.to_owned(),
            external_id: self.id.external_id.to_owned(),
            existing_version: None,
            r#type: None,
            sources: Some(vec![EdgeOrNodeData {
                source: SourceReference::View(
                    self.view
                        .unwrap_or(ViewReference {
                            space: Self::SPACE.to_string(),
                            external_id: Self::EXTERNAL_ID.to_string(),
                            version: Self::VERSION.to_string(),
                        })
                        .to_owned(),
                ),
                properties: self.file_object,
            }]),
        })
    }
}

impl FromReadable<FileObject> for CogniteExtractorFile {
    fn try_from(
        value: NodeOrEdge<FileObject>,
        view: Option<&ViewReference>,
    ) -> crate::Result<CogniteExtractorFile> {
        match value {
            NodeOrEdge::Node(node_definition) => {
                let mut properties = node_definition
                    .properties
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                let file_object: &FileObject = get_instance_properties(
                    view.unwrap_or(&ViewReference {
                        space: Self::SPACE.to_string(),
                        external_id: Self::EXTERNAL_ID.to_string(),
                        version: Self::VERSION.to_string(),
                    }),
                    &mut properties,
                )
                .ok_or(Error::Other("Invalid properties".to_string()))?;
                Ok(CogniteExtractorFile {
                    view: view.map(|v| v.to_owned()),
                    id: InstanceId {
                        external_id: node_definition.external_id,
                        space: node_definition.space,
                    },
                    audit: CogniteAuditable {
                        created_time: node_definition.created_time,
                        last_updated_time: node_definition.last_updated_time,
                        deleted_time: node_definition.deleted_time,
                    },
                    file_object: file_object.to_owned(),
                })
            }
            _ => Err(Error::Other("Invalid type".to_string())),
        }
    }
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
