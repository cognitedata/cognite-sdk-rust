use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dto::data_modeling::instances::SlimNodeOrEdge;
use crate::models::{
    AggregateInstancesRequest, AggregateInstancesResponse, FilterInstancesRequest,
    InstancesFilterResponse, NodeAndEdgeCreateCollection, NodeAndEdgeRetrieveRequest,
    NodeAndEdgeRetrieveResponse, NodeOrEdge, NodeOrEdgeSpecification, QueryInstancesRequest,
    QueryInstancesResponse, SearchInstancesRequest,
};
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
    pub async fn filter_with_type_info<TProperties: DeserializeOwned + Send + Sync + 'static>(
        &self,
        req: FilterInstancesRequest,
    ) -> Result<InstancesFilterResponse<TProperties>> {
        self.api_client
            .post(&format!("{}/list", Self::BASE_PATH), &req)
            .await
    }

    /// Perform a complex query against data models.
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
    pub async fn sync<TProperties: DeserializeOwned + Send + Sync + 'static>(
        &self,
        query: QueryInstancesRequest,
    ) -> Result<QueryInstancesResponse<TProperties>> {
        self.api_client
            .post(&format!("{}/sync", Self::BASE_PATH), &query)
            .await
    }

    /// Aggregate nodes and edges.
    pub async fn aggregate(
        &self,
        req: AggregateInstancesRequest,
    ) -> Result<AggregateInstancesResponse> {
        self.api_client
            .post(&format!("{}/aggregate", Self::BASE_PATH), &req)
            .await
    }

    /// Search nodes and edges.
    pub async fn search<TProperties: DeserializeOwned + Send + Sync + 'static>(
        &self,
        req: SearchInstancesRequest,
    ) -> Result<NodeAndEdgeRetrieveResponse<TProperties>> {
        self.api_client
            .post(&format!("{}/search", Self::BASE_PATH), &req)
            .await
    }
}
