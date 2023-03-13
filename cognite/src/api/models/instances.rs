use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dto::models::instances::SlimNodeOrEdge;
use crate::{dto::models::instances::Instance, Resource, WithBasePath};
use crate::{Items, ItemsWithCursor, Result, WithApiClient};

pub type Instances<TNodeOrEdgeProperties> = Resource<Instance<TNodeOrEdgeProperties>>;

impl<TNodeOrEdgeProperties> WithBasePath for Instances<TNodeOrEdgeProperties> {
    const BASE_PATH: &'static str = "models/instances";
}

impl<TNodeOrEdgeProperties> Instances<TNodeOrEdgeProperties> {
    pub async fn add(&self, instances: &[Instance<TNodeOrEdgeProperties>]) -> Result<Vec<SlimNodeOrEdge>>
    where
        TNodeOrEdgeProperties: Serialize + Sync + Send,
        Self: WithApiClient + WithBasePath,
    {
        let items = Items::from(instances);
        let response: ItemsWithCursor<SlimNodeOrEdge> =
            self.get_client().post(Self::BASE_PATH, &items).await?;
        Ok(response.items)
    }

    pub async fn retrieve(
        &self,
        instance_types: &[String],
        external_ids: &[i64],
        spaces: &[String],
    ) -> Result<Vec<Instance<TNodeOrEdgeProperties>>>
    where
        TNodeOrEdgeProperties: DeserializeOwned + Sync + Send,
        Self: WithApiClient + WithBasePath,
    {
        if 


        let id_items = RetrieveInstancesRequest {
            instance_type: instance_types.to_vec(),
            external_id: external_ids.to_vec(),
            space: spaces.to_vec(),
        };
        self.get_client()
            .post(&format!("{}/byids", Self::BASE_PATH), &id_items)
            .await?;
        Ok(response.items)
    }
}

pub struct RetrieveInstancesRequest {
    pub instance_type: String,
    pub external_id: String,
    pub space: String,
}
