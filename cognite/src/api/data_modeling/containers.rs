use crate::{
    dto::data_modeling::containers::{
        ContainerComponentId, ContainerCreate, ContainerDefinition, ContainerQuery,
    },
    models::ItemId,
    Create, DeleteWithResponse, Items, ItemsWithoutCursor, List, Resource, Result, Retrieve,
    WithBasePath,
};

pub struct Container {}
pub type Containers = Resource<Container>;

impl WithBasePath for Containers {
    const BASE_PATH: &'static str = "models/containers";
}

impl Create<ContainerCreate, ContainerDefinition> for Containers {}
impl DeleteWithResponse<ItemId, ItemId> for Containers {}
impl List<ContainerQuery, ContainerDefinition> for Containers {}
impl Retrieve<ItemId, ContainerDefinition> for Containers {}

impl Containers {
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
