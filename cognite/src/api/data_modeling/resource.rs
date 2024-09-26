use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::data_models::{FromReadable, IntoWritable};
use crate::models::instances::{
    NodeAndEdgeCreateCollection, NodeAndEdgeRetrieveRequest, NodeAndEdgeRetrieveResponse,
    NodeOrEdgeCreate, NodeOrEdgeSpecification, SlimNodeOrEdge, SourceReferenceInternal,
};
use crate::models::views::ViewReference;
use crate::{Result, RetrieveWithRequest, UpsertCollection};

use super::instances::Instances;
/// Data model files.
pub mod files;

/// 
pub trait WithView {
    /// Default space
    const SPACE: &'static str;
    /// Default external ID
    const EXTERNAL_ID: &'static str;
    /// Default version
    const VERSION: &'static str;

    fn with_view(&mut self, space: String, external_id: String, version: String);
    fn view(&self) -> ViewReference;
}

pub struct DataModelsResource {
    pub instance_resource: Instances,
    view: Option<ViewReference>,
}

impl DataModelsResource {
    pub fn new(instances: Instances) -> Self {
        Self {
            instance_resource: instances,
            view: None,
        }
    }
}

pub trait WithInstanceApiResource {
    fn get_resource(&self) -> &Instances;
}

impl WithInstanceApiResource for DataModelsResource {
    fn get_resource(&self) -> &Instances {
        &self.instance_resource
    }
}

pub trait RetrieveExtendedCollection<TProperties, TEntity>
where
    Self: WithView + WithInstanceApiResource,
    TProperties: Serialize + DeserializeOwned + Send + Sync,
    TEntity: FromReadable<TProperties> + Send,
{
    async fn retrieve(&self, items: Vec<NodeOrEdgeSpecification>) -> Result<Vec<TEntity>> {
        let response: NodeAndEdgeRetrieveResponse<TProperties> = self
            .get_resource()
            .retrieve(&NodeAndEdgeRetrieveRequest {
                sources: Some(vec![SourceReferenceInternal {
                    source: self.view().into(),
                }]),
                items,
                include_typing: None,
            })
            .await?;
        response
            .items
            .into_iter()
            .map(|item| TEntity::try_from_readable(item, self.view()))
            .collect()
    }
}

pub trait UpsertExtendedCollection<TEntity, TProperties>
where
    Self: WithView + WithInstanceApiResource,
    TProperties: Serialize + DeserializeOwned + Send + Sync,
    TEntity: IntoWritable<TProperties> + Send,
{
    /// Upsert custom instance
    async fn upsert(
        &self,
        col: Vec<TEntity>,
        auto_create_direct_relations: Option<bool>,
        auto_create_start_nodes: Option<bool>,
        auto_create_end_nodes: Option<bool>,
        skip_on_version_conflict: Option<bool>,
        replace: Option<bool>,
    ) -> Result<Vec<SlimNodeOrEdge>>
    where
        TProperties: Serialize + Send + Sync,
    {
        let collection: Vec<NodeOrEdgeCreate<TProperties>> = col
            .into_iter()
            .map(|t| t.try_into_writable(self.view()))
            .collect::<Result<Vec<NodeOrEdgeCreate<_>>>>()?;

        let collection = NodeAndEdgeCreateCollection {
            items: collection,
            auto_create_direct_relations: auto_create_direct_relations.or(Some(true)),
            auto_create_start_nodes,
            auto_create_end_nodes,
            skip_on_version_conflict,
            replace,
        };
        self.get_resource().upsert(&collection).await
    }
}
