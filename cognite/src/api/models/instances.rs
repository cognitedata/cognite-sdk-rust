use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dto::models::instances::{NodeOrEdge, RetrieveInstancesRequest, SlimNodeOrEdge};
use crate::{dto::models::instances::Instance, Resource, WithBasePath};
use crate::{Error, Items, ItemsWithoutCursor, Result, WithApiClient};

pub type Instances<TNodeOrEdgeProperties> = Resource<NodeOrEdge<TNodeOrEdgeProperties>>;

impl<TNodeOrEdgeProperties> WithBasePath for Instances<TNodeOrEdgeProperties> {
    const BASE_PATH: &'static str = "models/instances";
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

    pub async fn retrieve(
        &self,
        instance_types: &[String],
        external_ids: &[i64],
        spaces: &[String],
    ) -> Result<Vec<NodeOrEdge<TNodeOrEdgeProperties>>>
    where
        TNodeOrEdgeProperties: Serialize + DeserializeOwned + Sync + Send,
        Self: WithApiClient + WithBasePath,
    {
        if !(instance_types.len() == external_ids.len() && instance_types.len() == spaces.len()) {
            return Err(Error::new_without_json(
                StatusCode::BAD_REQUEST,
                "instance_types, external_ids, and spaces must be the same length".to_string(),
                None,
            ));
        }

        let items = instance_types
            .iter()
            .zip(external_ids.iter())
            .zip(spaces.iter())
            .map(|((instance_type, external_id), space)| {
                RetrieveInstancesRequest::from((
                    instance_type.to_string(),
                    external_id.to_string(),
                    space.to_string(),
                ))
            })
            .collect::<Vec<RetrieveInstancesRequest>>();

        let response: ItemsWithoutCursor<NodeOrEdge<TNodeOrEdgeProperties>> = self
            .get_client()
            .post(&format!("{}/byids", Self::BASE_PATH), &Items::from(&items))
            .await?;
        Ok(response.items)
    }
}
