use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;

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

/// Trait for data models special instance
pub trait WithView {
    /// Default space
    const SPACE: &'static str;
    /// Default external ID
    const EXTERNAL_ID: &'static str;
    /// Default version
    const VERSION: &'static str;
    /// Get view for instance
    fn view(&self) -> ViewReference;
}

/// A data models instance resoure for special models.
pub struct DataModelsResource<T> {
    /// An instance resoure for special instances.
    pub instance_resource: Arc<Instances>,
    /// View for the resources of this instance
    pub view: Option<ViewReference>,
    _marker: PhantomData<T>
}

impl<T> DataModelsResource<T> {
    /// Create a new data models instance resource.
    ///
    /// # Arguments
    ///
    /// * `instances` - A shared instance resource.
    pub fn new(instances: Arc<Instances>) -> Self {
        Self {
            instance_resource: instances,
            view: None,
            _marker: PhantomData
        }
    }
}

/// Trait for a type that contains an instance resource with client.
pub trait WithInstanceApiResource {
    /// Get instance resource for this type.
    fn get_resource(&self) -> &Instances;
}

impl<T> WithInstanceApiResource for DataModelsResource<T> {
    fn get_resource(&self) -> &Instances {
        &self.instance_resource
    }
}

/// Trait for retieving a list of data models instances of this type.
pub trait RetrieveExtendedCollection<TProperties, TEntity>
where
    Self: WithView + WithInstanceApiResource + Sync,
    TProperties: Serialize + DeserializeOwned + Send + Sync,
    TEntity: FromReadable<TProperties> + Send,
{
    /// Fetch special data models instance collection.
    ///
    /// # Arguments
    ///
    /// * `items` - A list of specifications of node/edges to retrieve.
    fn retrieve(
        &self,
        items: Vec<NodeOrEdgeSpecification>,
    ) -> impl Future<Output = Result<Vec<TEntity>>> + Send {
        async move {
            let response: NodeAndEdgeRetrieveResponse<TProperties> = self
                .get_resource()
                .retrieve(&NodeAndEdgeRetrieveRequest {
                    sources: Some(vec![SourceReferenceInternal {
                        source: self.view().clone().into(),
                    }]),
                    items,
                    include_typing: None,
                })
                .await?;
            response
                .items
                .into_iter()
                .map(|item| TEntity::try_from_readable(item, self.view().clone()))
                .collect()
        }
    }
}

/// Trait for creating a list of data models instances of this type.
pub trait UpsertExtendedCollection<TEntity, TProperties>
where
    Self: WithView + WithInstanceApiResource + Sync,
    TProperties: Serialize + DeserializeOwned + Send + Sync,
    TEntity: IntoWritable<TProperties> + Send,
{
    /// Upsert data models instances of this type.
    ///
    /// # Arguments
    ///
    /// * `col` - A list of this type to be created.
    /// * `auto_create_direct_relation` - Whether to auto create direct relation that do no exist.
    /// * `auto_create_start_nodes` - Whether to auto create end nodes that do not exist.
    /// * `auto_create_end_nodes` - Whether to auto create end nodes that do not exist.
    /// * `skip_on_version_conflict` - Whether to skip when a version conflic is encountered.
    /// * `replace` - Whether to replace all matching and existing values with the supplied values.
    fn upsert(
        &self,
        col: Vec<TEntity>,
        auto_create_direct_relations: Option<bool>,
        auto_create_start_nodes: Option<bool>,
        auto_create_end_nodes: Option<bool>,
        skip_on_version_conflict: Option<bool>,
        replace: Option<bool>,
    ) -> impl Future<Output = Result<Vec<SlimNodeOrEdge>>> + Send
    where
        TProperties: Serialize + Send + Sync,
    {
        async move {
            let collection: Vec<NodeOrEdgeCreate<TProperties>> = col
                .into_iter()
                .map(|t| t.try_into_writable(self.view().clone()))
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
}
