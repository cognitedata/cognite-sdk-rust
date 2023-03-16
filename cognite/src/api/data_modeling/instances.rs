use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dto::data_modeling::instances::{NodeOrEdge, SlimNodeOrEdge};
use crate::models::{
    InstanceId, InstancesFilter, NodeAndEdgeCreateCollection, NodeOrEdgeSpecification,
};
use crate::{
    DeleteWithResponse, Filter, FilterWithRequest, ItemsWithCursor, Retrieve, UpsertCollection,
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
    TProperties: Serialize + DeserializeOwned + Sync + Send,
{
}
impl<TProperties> Retrieve<NodeOrEdgeSpecification, NodeOrEdge<TProperties>> for Instances where
    TProperties: Serialize + DeserializeOwned + Sync + Send
{
}
impl<TProperties> UpsertCollection<NodeAndEdgeCreateCollection<TProperties>, SlimNodeOrEdge>
    for Instances
where
    TProperties: Serialize + Sync + Send,
{
}
impl DeleteWithResponse<NodeOrEdgeSpecification, InstanceId> for Instances {}
