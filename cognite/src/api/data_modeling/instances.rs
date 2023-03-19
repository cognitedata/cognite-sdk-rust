use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dto::data_modeling::instances::SlimNodeOrEdge;
use crate::models::{
    FilterInstancesRequest, NodeAndEdgeCreateCollection, NodeAndEdgeRetrieveRequest,
    NodeAndEdgeRetrieveResponse, NodeOrEdge, NodeOrEdgeSpecification,
};
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
