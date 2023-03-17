use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dto::data_modeling::instances::{NodeOrEdge, SlimNodeOrEdge};
use crate::models::{
    InstancesFilter, NodeAndEdgeCreateCollection, NodeAndEdgeRetrieveResponse,
    NodeOrEdgeSpecification,
};
use crate::{
    DeleteWithResponse, Filter, FilterWithRequest, ItemsWithCursor, RetrieveWithRequest,
    UpsertCollection,
};
use crate::{Resource, WithBasePath};

pub struct Instance {}
pub type Instances = Resource<Instance>;

impl WithBasePath for Instances {
    const BASE_PATH: &'static str = "models/instances";
}

impl<TProperties>
    FilterWithRequest<Filter<InstancesFilter>, ItemsWithCursor<NodeOrEdge<TProperties>>>
    for Instances
where
    TProperties: Serialize + DeserializeOwned + Send + Sync,
{
}

impl<TProperties>
    RetrieveWithRequest<
        NodeAndEdgeRetrieveResponse<TProperties>,
        NodeAndEdgeRetrieveResponse<TProperties>,
    > for Instances
where
    TProperties: Serialize + DeserializeOwned + Send + Sync,
{
}
impl<TProperties> UpsertCollection<NodeAndEdgeCreateCollection<TProperties>, SlimNodeOrEdge>
    for Instances
where
    TProperties: Serialize + DeserializeOwned + Send + Sync,
{
}
impl DeleteWithResponse<NodeOrEdgeSpecification, NodeOrEdgeSpecification> for Instances {}
