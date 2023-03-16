use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dto::data_modeling::instances::{InstanceInfo, NodeOrEdge, SlimNodeOrEdge};
use crate::models::{InstanceId, InstancesFilter, NodeAndEdgeCreateCollection};
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
impl<TProperties> Retrieve<InstanceInfo, NodeOrEdge<TProperties>> for Instances where
    TProperties: Serialize + DeserializeOwned + Sync + Send
{
}
impl<TProperties> UpsertCollection<NodeAndEdgeCreateCollection<TProperties>, SlimNodeOrEdge>
    for Instances
where
    TProperties: Serialize + Sync + Send,
{
}
impl DeleteWithResponse<InstanceInfo, InstanceId> for Instances {}
