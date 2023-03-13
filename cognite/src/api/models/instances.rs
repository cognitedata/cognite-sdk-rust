use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dto::models::instances::{InstanceItemInfo, NodeOrEdge, SlimNodeOrEdge};
use crate::{dto::models::instances::Instance, Resource, WithBasePath};
use crate::{Delete, Items, ItemsWithoutCursor, Result, Retrieve, WithApiClient};

pub type Instances<TNodeOrEdgeProperties> = Resource<NodeOrEdge<TNodeOrEdgeProperties>>;

impl<TNodeOrEdgeProperties> WithBasePath for Instances<TNodeOrEdgeProperties> {
    const BASE_PATH: &'static str = "models/instances";
}

impl<TNodeOrEdgeProperties> Delete<InstanceItemInfo> for Instances<TNodeOrEdgeProperties> {}
impl<TNodeOrEdgeProperties> Retrieve<InstanceItemInfo, NodeOrEdge<TNodeOrEdgeProperties>>
    for Instances<TNodeOrEdgeProperties>
where
    TNodeOrEdgeProperties: Serialize + DeserializeOwned + Sync + Send,
    Self: WithApiClient + WithBasePath,
{
}

impl<TNodeOrEdgeProperties> Instances<TNodeOrEdgeProperties> {
    pub async fn upsert(
        &self,
        instances: &[Instance<TNodeOrEdgeProperties>],
    ) -> Result<Vec<SlimNodeOrEdge>>
    where
        TNodeOrEdgeProperties: Serialize + Sync + Send,
        Self: WithApiClient + WithBasePath,
    {
        let items = Items::from(instances);
        let response: ItemsWithoutCursor<SlimNodeOrEdge> =
            self.get_client().post(Self::BASE_PATH, &items).await?;
        Ok(response.items)
    }
}
