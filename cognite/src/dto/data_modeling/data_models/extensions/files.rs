use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    models::{
        instances::{EdgeOrNodeData, InstanceId, NodeDefinition, NodeWrite},
        views::ViewReference,
        SourceReference,
    },
    Error,
};

use super::{FromReadable, IntoWritable};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CogniteExtractorFile<TExtractedData: Serialize + Send + Sync> {
    #[serde(skip_serializing)]
    pub space: String,
    #[serde(skip_serializing)]
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
    pub extracted_data: Option<TExtractedData>,
}

impl<TExtractedData> From<CogniteExtractorFile<TExtractedData>> for FileProperties<TExtractedData>
where
    TExtractedData: Serialize + Send + Sync,
{
    fn from(value: CogniteExtractorFile<TExtractedData>) -> Self {
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

impl<TExtractedData> IntoWritable<FileProperties<TExtractedData>>
    for CogniteExtractorFile<TExtractedData>
where
    TExtractedData: Serialize + Send + Sync,
{
    fn try_into_writable(
        self,
        view: ViewReference,
    ) -> crate::Result<NodeWrite<FileProperties<TExtractedData>>> {
        Ok(NodeWrite {
            space: self.space.to_owned(),
            external_id: self.external_id.to_owned(),
            existing_version: None,
            r#type: None,
            sources: Some(vec![EdgeOrNodeData {
                source: SourceReference::View(view),
                properties: self.into(),
            }]),
        })
    }
}

impl<TExtractedData> FromReadable<NodeDefinition<FileProperties<TExtractedData>>>
    for CogniteExtractorFile<TExtractedData>
where
    TExtractedData: Serialize + Send + Sync,
{
    fn try_from_node_definition(
        mut value: NodeDefinition<FileProperties<TExtractedData>>,
        view: ViewReference,
    ) -> crate::Result<CogniteExtractorFile<TExtractedData>> {
        // TODO: make error better
        let mut properties = value
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
            external_id: value.external_id,
            space: value.space,
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
}

#[derive(Serialize, Deserialize)]
pub struct FileProperties<TExtractedData: Serialize + Send + Sync> {
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
    extracted_data: Option<TExtractedData>,
}
