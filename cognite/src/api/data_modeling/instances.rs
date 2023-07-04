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

pub struct Instance {}
pub type Instances = Resource<Instance>;

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
    pub async fn filter_with_type_info<TProperties: DeserializeOwned + Send + Sync + 'static>(
        &self,
        req: FilterInstancesRequest,
    ) -> Result<InstancesFilterResponse<TProperties>> {
        self.api_client
            .post(&format!("{}/list", Self::BASE_PATH), &req)
            .await
    }

    pub async fn query<TProperties: DeserializeOwned + Send + Sync + 'static>(
        &self,
        query: QueryInstancesRequest,
    ) -> Result<QueryInstancesResponse<TProperties>> {
        self.api_client
            .post(&format!("{}/query", Self::BASE_PATH), &query)
            .await
    }

    pub async fn sync<TProperties: DeserializeOwned + Send + Sync + 'static>(
        &self,
        query: QueryInstancesRequest,
    ) -> Result<QueryInstancesResponse<TProperties>> {
        self.api_client
            .post(&format!("{}/sync", Self::BASE_PATH), &query)
            .await
    }

    pub async fn aggregate(
        &self,
        req: AggregateInstancesRequest,
    ) -> Result<AggregateInstancesResponse> {
        self.api_client
            .post(&format!("{}/aggregate", Self::BASE_PATH), &req)
            .await
    }

    pub async fn search<TProperties: DeserializeOwned + Send + Sync + 'static>(
        &self,
        req: SearchInstancesRequest,
    ) -> Result<NodeAndEdgeRetrieveResponse<TProperties>> {
        self.api_client
            .post(&format!("{}/search", Self::BASE_PATH), &req)
            .await
    }
}
