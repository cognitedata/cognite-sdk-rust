use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    models::{
        instances::{EdgeOrNodeData, InstanceId, NodeOrEdge, NodeOrEdgeCreate, NodeWrite},
        views::ViewReference,
        SourceReference,
    },
    Error,
};

use super::{FromReadable, IntoWritable};

#[derive(Clone, Debug, Default)]
pub struct CogniteExtractorFile {
    pub space: String,
    pub external_id: String,
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub aliases: Option<Vec<String>>,
    pub source_id: Option<String>,
    pub source_context: Option<String>,
    pub source: Option<InstanceId>,
    pub source_created_time: Option<i64>,
    pub source_updated_time: Option<i64>,
    pub source_created_user: Option<String>,
    pub source_updated_user: Option<String>,
    pub assets: Option<Vec<InstanceId>>,
    pub mime_type: Option<String>,
    pub directory: Option<String>,
    pub is_uploaded: Option<bool>,
    pub uploaded_time: Option<i64>,
    pub category: Option<InstanceId>,
    pub extracted_data: Option<HashMap<String, String>>,
}

impl From<CogniteExtractorFile> for FileProperties {
    fn from(value: CogniteExtractorFile) -> Self {
        Self {
            name: value.name,
            description: value.description,
            tags: value.tags,
            aliases: value.aliases,
            source_id: value.source_id,
            source_context: value.source_context,
            source: value.source,
            source_created_time: value.source_created_time,
            source_updated_time: value.source_updated_time,
            source_created_user: value.source_created_user,
            source_updated_user: value.source_updated_user,
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
    pub fn new(space: String, external_id: String, name: String) -> Self {
        CogniteExtractorFile {
            name,
            space,
            external_id,
            ..Default::default()
        }
    }
}

impl IntoWritable<FileProperties> for CogniteExtractorFile {
    fn try_into_writable(
        self,
        view: ViewReference,
    ) -> crate::Result<NodeOrEdgeCreate<FileProperties>> {
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

impl FromReadable<FileProperties> for CogniteExtractorFile {
    fn try_from_readable(
        value: NodeOrEdge<FileProperties>,
        view: ViewReference,
    ) -> crate::Result<CogniteExtractorFile> {
        // TODO: make error better
        match value {
            NodeOrEdge::Node(mut node_definition) => {
                let mut properties = node_definition
                    .properties
                    .take()
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                let main_prop_key = view.space;
                let sub_prop_key = format!("{}/{}", view.external_id, view.version);
                let main_prop = properties
                    .get_mut(&main_prop_key)
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                let sub_prop = main_prop
                    .get_mut(&sub_prop_key)
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                Ok(CogniteExtractorFile {
                    external_id: node_definition.external_id,
                    space: node_definition.space,
                    name: sub_prop.name.clone(),
                    description: sub_prop.description.clone(),
                    tags: sub_prop.tags.clone(),
                    aliases: sub_prop.aliases.clone(),
                    source_id: sub_prop.source_id.clone(),
                    source_context: sub_prop.source_context.clone(),
                    source: sub_prop.source.clone(),
                    source_created_time: sub_prop.source_created_time,
                    source_updated_time: sub_prop.source_updated_time,
                    source_created_user: sub_prop.source_created_user.clone(),
                    source_updated_user: sub_prop.source_updated_user.clone(),
                    assets: sub_prop.assets.clone(),
                    mime_type: sub_prop.mime_type.clone(),
                    directory: sub_prop.directory.clone(),
                    is_uploaded: sub_prop.is_uploaded,
                    uploaded_time: sub_prop.uploaded_time,
                    category: sub_prop.category.clone(),
                    extracted_data: sub_prop.extracted_data.take(),
                })
            }
            _ => Err(Error::Other("Invalid type".to_string())),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileProperties {
    name: String,
    description: Option<String>,
    tags: Option<Vec<String>>,
    aliases: Option<Vec<String>>,
    source_id: Option<String>,
    source_context: Option<String>,
    source: Option<InstanceId>,
    source_created_time: Option<i64>,
    source_updated_time: Option<i64>,
    source_created_user: Option<String>,
    source_updated_user: Option<String>,
    assets: Option<Vec<InstanceId>>,
    mime_type: Option<String>,
    directory: Option<String>,
    is_uploaded: Option<bool>,
    uploaded_time: Option<i64>,
    category: Option<InstanceId>,
    extracted_data: Option<HashMap<String, String>>,
}
