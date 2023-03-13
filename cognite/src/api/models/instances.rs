use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dto::models::instances::{InstanceInfo, ListRequest, NodeOrEdge, SlimNodeOrEdge};
use crate::{dto::models::instances::Instance, Resource, WithBasePath};
use crate::{Delete, ItemsWithCursor, ListWithRequest, Retrieve, Upsert};

pub type Instances<TNodeOrEdgeProperties> = Resource<NodeOrEdge<TNodeOrEdgeProperties>>;

impl<TNodeOrEdgeProperties> WithBasePath for Instances<TNodeOrEdgeProperties> {
    const BASE_PATH: &'static str = "models/instances";
}

impl<TNodeOrEdgeProperties>
    ListWithRequest<ItemsWithCursor<NodeOrEdge<TNodeOrEdgeProperties>>, ListRequest>
    for Instances<TNodeOrEdgeProperties>
where
    TNodeOrEdgeProperties: Serialize + DeserializeOwned + Sync + Send,
{
}
impl<TNodeOrEdgeProperties> Delete<InstanceInfo> for Instances<TNodeOrEdgeProperties> {}
impl<TNodeOrEdgeProperties> Retrieve<InstanceInfo, NodeOrEdge<TNodeOrEdgeProperties>>
    for Instances<TNodeOrEdgeProperties>
where
    TNodeOrEdgeProperties: Serialize + DeserializeOwned + Sync + Send,
{
}
impl<TNodeOrEdgeProperties> Upsert<Instance<TNodeOrEdgeProperties>, SlimNodeOrEdge>
    for Instances<TNodeOrEdgeProperties>
where
    TNodeOrEdgeProperties: Serialize + Sync + Send,
{
}
