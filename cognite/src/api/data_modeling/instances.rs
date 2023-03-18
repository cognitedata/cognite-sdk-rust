use crate::dto::data_modeling::instances::{NodeOrEdge, SlimNodeOrEdge};
use crate::models::{
    InstancesFilter, NodeAndEdgeCreateCollection, NodeAndEdgeRetrieveRequest,
    NodeAndEdgeRetrieveResponse, NodeOrEdgeSpecification,
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

impl FilterWithRequest<Filter<InstancesFilter>, ItemsWithCursor<NodeOrEdge>> for Instances {}
impl RetrieveWithRequest<NodeAndEdgeRetrieveRequest, NodeAndEdgeRetrieveResponse> for Instances {}
impl<TProperties> UpsertCollection<NodeAndEdgeCreateCollection<TProperties>, SlimNodeOrEdge>
    for Instances
{
}
impl DeleteWithResponse<NodeOrEdgeSpecification, NodeOrEdgeSpecification> for Instances {}
