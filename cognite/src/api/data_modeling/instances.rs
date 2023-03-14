use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dto::data_modeling::instances::{
    InstanceInfo, ListRequest, NodeOrEdge, NodeOrEdgeCreate, SlimNodeOrEdge,
};
use crate::{Delete, ItemsWithCursor, ListWithRequest, Retrieve, Upsert};
use crate::{Resource, WithBasePath};

pub struct Instance {}
pub type Instances = Resource<Instance>;

impl WithBasePath for Instances {
    const BASE_PATH: &'static str = "models/instances";
}

impl<TProperties> ListWithRequest<ItemsWithCursor<NodeOrEdge<TProperties>>, ListRequest>
    for Instances
where
    TProperties: Serialize + DeserializeOwned + Sync + Send,
{
}
impl<TProperties> Retrieve<InstanceInfo, NodeOrEdge<TProperties>> for Instances where
    TProperties: Serialize + DeserializeOwned + Sync + Send
{
}
impl<TProperties> Upsert<NodeOrEdgeCreate<TProperties>, SlimNodeOrEdge> for Instances where
    TProperties: Serialize + Sync + Send
{
}
impl Delete<InstanceInfo> for Instances {}
