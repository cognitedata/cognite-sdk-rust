use crate::{
    dto::data_modeling::containers::{
        ContainerComponentId, ContainerCreate, ContainerDefinition, ContainerQuery,
    },
    models::ItemId,
    Create, DeleteWithResponse, Items, ItemsWithoutCursor, List, Resource, Result, Retrieve,
    WithBasePath,
};

/// A container represents a bag of properties, each property has a type.
/// Containers can have indexes, constraints, and default values.
pub type ContainersResource = Resource<ContainerDefinition>;

impl WithBasePath for ContainersResource {
    const BASE_PATH: &'static str = "models/containers";
}

impl Create<ContainerCreate, ContainerDefinition> for ContainersResource {}
impl DeleteWithResponse<ItemId, ItemId> for ContainersResource {}
impl List<ContainerQuery, ContainerDefinition> for ContainersResource {}
impl Retrieve<ItemId, ContainerDefinition> for ContainersResource {}

impl ContainersResource {
    pub async fn delete_constraints(
        &self,
        items: &[ContainerComponentId],
    ) -> Result<Vec<ContainerComponentId>> {
        let r: ItemsWithoutCursor<ContainerComponentId> = self
            .api_client
            .post(
                &format!("{}/constraints/delete", Self::BASE_PATH),
                &Items::from(items),
            )
            .await?;
        Ok(r.items)
    }

    pub async fn delete_indexes(
        &self,
        items: &[ContainerComponentId],
    ) -> Result<Vec<ContainerComponentId>> {
        let r: ItemsWithoutCursor<ContainerComponentId> = self
            .api_client
            .post(
                &format!("{}/indexes/delete", Self::BASE_PATH),
                &Items::from(items),
            )
            .await?;
        Ok(r.items)
    }
}
