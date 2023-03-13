use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dto::data_modeling::instances::{
    InstanceInfo, ListRequest, NodeOrEdge, NodeOrEdgeWrite, SlimNodeOrEdge,
};
use crate::{Delete, ItemsWithCursor, ListWithRequest, Retrieve, Upsert};
use crate::{Resource, WithBasePath};

pub struct Instance {}
pub type Instances = Resource<Instance>;

impl WithBasePath for Instances {
    const BASE_PATH: &'static str = "models/instances";
}

impl<TNodeOrEdgeProperties>
    ListWithRequest<ItemsWithCursor<NodeOrEdge<TNodeOrEdgeProperties>>, ListRequest> for Instances
where
    TNodeOrEdgeProperties: Serialize + DeserializeOwned + Sync + Send,
{
}
impl<TNodeOrEdgeProperties> Retrieve<InstanceInfo, NodeOrEdge<TNodeOrEdgeProperties>> for Instances where
    TNodeOrEdgeProperties: Serialize + DeserializeOwned + Sync + Send
{
}
impl<TNodeOrEdgeProperties> Upsert<NodeOrEdgeWrite<TNodeOrEdgeProperties>, SlimNodeOrEdge>
    for Instances
where
    TNodeOrEdgeProperties: Serialize + Sync + Send,
{
}
impl Delete<InstanceInfo> for Instances {}
