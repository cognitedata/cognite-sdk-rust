use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dto::data_modeling::instances::SlimNodeOrEdge;
use crate::models::instances::{
    AggregateInstancesRequest, AggregateInstancesResponse, FilterInstancesRequest,
    InstancesFilterResponse, NodeAndEdgeCreateCollection, NodeAndEdgeRetrieveRequest,
    NodeAndEdgeRetrieveResponse, NodeOrEdge, NodeOrEdgeCreate, NodeOrEdgeSpecification,
    QueryInstancesRequest, QueryInstancesResponse, SearchInstancesRequest, SourceReferenceInternal,
};
use crate::models::instances::{FromReadable, WithView};
use crate::models::views::ViewReference;
use crate::Result;
use crate::{DeleteWithResponse, FilterWithRequest, RetrieveWithRequest, UpsertCollection};
use crate::{Resource, WithBasePath};

/// Instances are nodes and edges in a data model. These contain the actual data in the data model.
pub type Instances = Resource<SlimNodeOrEdge>;

impl WithBasePath for Instances {
    const BASE_PATH: &'static str = "models/instances";
}

impl<TProperties> FilterWithRequest<FilterInstancesRequest, NodeOrEdge<TProperties>> for Instances where
    TProperties: Serialize + DeserializeOwned + Send + Sync
{
}
impl<TProperties>
    RetrieveWithRequest<NodeAndEdgeRetrieveRequest, NodeAndEdgeRetrieveResponse<TProperties>>
    for Instances
where
    TProperties: Serialize + DeserializeOwned + Send + Sync,
{
}
impl<TProperties> UpsertCollection<NodeAndEdgeCreateCollection<TProperties>, SlimNodeOrEdge>
    for Instances
{
}
impl DeleteWithResponse<NodeOrEdgeSpecification, NodeOrEdgeSpecification> for Instances {}

impl Instances {
    /// Filter instances optionally returning type information.
    ///
    /// # Arguments
    ///
    /// * `req` - Request with optional filter.
    pub async fn filter_with_type_info<TProperties: DeserializeOwned + Send + Sync + 'static>(
        &self,
        req: FilterInstancesRequest,
    ) -> Result<InstancesFilterResponse<TProperties>> {
        self.api_client
            .post(&format!("{}/list", Self::BASE_PATH), &req)
            .await
    }

    /// Perform a complex query against data models.
    ///
    /// # Arguments
    ///
    /// * `query` - Query to execute.
    pub async fn query<TProperties: DeserializeOwned + Send + Sync + 'static>(
        &self,
        query: QueryInstancesRequest,
    ) -> Result<QueryInstancesResponse<TProperties>> {
        self.api_client
            .post(&format!("{}/query", Self::BASE_PATH), &query)
            .await
    }

    /// Perform a complex query against data models. This always returns cursors,
    /// so you can keep querying to get any changes since the last query.
    ///
    /// # Arguments
    ///
    /// * `query` - Query to execute.
    pub async fn sync<TProperties: DeserializeOwned + Send + Sync + 'static>(
        &self,
        query: QueryInstancesRequest,
    ) -> Result<QueryInstancesResponse<TProperties>> {
        self.api_client
            .post(&format!("{}/sync", Self::BASE_PATH), &query)
            .await
    }

    /// Aggregate nodes and edges.
    ///
    /// # Arguments
    ///
    /// * `req` - Aggregates to compute.
    pub async fn aggregate(
        &self,
        req: AggregateInstancesRequest,
    ) -> Result<AggregateInstancesResponse> {
        self.api_client
            .post(&format!("{}/aggregate", Self::BASE_PATH), &req)
            .await
    }

    /// Search nodes and edges.
    ///
    /// # Arguments
    ///
    /// * `req` - Search request.
    pub async fn search<TProperties: DeserializeOwned + Send + Sync + 'static>(
        &self,
        req: SearchInstancesRequest,
    ) -> Result<NodeAndEdgeRetrieveResponse<TProperties>> {
        self.api_client
            .post(&format!("{}/search", Self::BASE_PATH), &req)
            .await
    }

    /// Fetch special data models instance collection.
    ///
    /// # Arguments
    ///
    /// * `items` - A list of specifications of node/edges to retrieve.
    pub async fn fetch<TEntity, TProperties>(
        &self,
        items: &[NodeOrEdgeSpecification],
        view: Option<&ViewReference>,
    ) -> Result<Vec<TEntity>>
    where
        TProperties: Serialize + DeserializeOwned + Send + Sync,
        TEntity: FromReadable<TProperties> + WithView + Send,
    {
        let response: NodeAndEdgeRetrieveResponse<TProperties> = self
            .retrieve(&NodeAndEdgeRetrieveRequest {
                sources: Some(vec![SourceReferenceInternal {
                    source: view
                        .unwrap_or(&ViewReference {
                            space: TEntity::SPACE.to_owned(),
                            external_id: TEntity::EXTERNAL_ID.to_owned(),
                            version: TEntity::VERSION.to_owned(),
                        })
                        .to_owned()
                        .into(),
                }]),
                items: items.to_vec(),
                include_typing: None,
            })
            .await?;
        response
            .items
            .into_iter()
            .map(|item| TEntity::try_from(item, view))
            .collect()
    }

    /// Upsert data models instances of this type.
    ///
    /// # Arguments
    ///
    /// * `col` - A list of this type to be created.
    /// * `auto_create_direct_relation` - Whether to auto create direct relation that do no exist.
    /// * `auto_create_start_nodes` - Whether to auto create end nodes that do not exist.
    /// * `auto_create_end_nodes` - Whether to auto create end nodes that do not exist.
    /// * `skip_on_version_conflict` - Whether to skip when a version conflict is encountered.
    /// * `replace` - Whether to replace all matching and existing values with the supplied values.
    pub async fn apply<TEntity, TProperties>(
        &self,
        col: &[TEntity],
        auto_create_direct_relations: Option<bool>,
        auto_create_start_nodes: Option<bool>,
        auto_create_end_nodes: Option<bool>,
        skip_on_version_conflict: Option<bool>,
        replace: bool,
    ) -> Result<Vec<SlimNodeOrEdge>>
    where
        TProperties: Serialize + DeserializeOwned + Send + Sync,
        TEntity: Clone + Into<NodeOrEdgeCreate<TProperties>> + Send,
    {
        let collection = col
            .iter()
            .map(|t| t.to_owned().into())
            .collect::<Vec<NodeOrEdgeCreate<_>>>();

        let collection = NodeAndEdgeCreateCollection {
            items: collection,
            auto_create_direct_relations: auto_create_direct_relations.or(Some(true)),
            auto_create_start_nodes,
            auto_create_end_nodes,
            skip_on_version_conflict,
            replace: Some(replace),
        };
        self.upsert(&collection).await
    }
}
