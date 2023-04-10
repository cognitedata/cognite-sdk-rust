use crate::dto::data_modeling::containers::{
    ContainerCreateDefinition, ContainerDefinition, ContainerListQuery, ContainerReference,
    ContainerRetrieveQuery,
};
use crate::{
    Create, DeleteWithResponse, List, Resource, Retrieve, RetrieveWithQuery, WithBasePath,
};

pub struct Container {}
pub type Containers = Resource<Container>;

impl WithBasePath for Containers {
    const BASE_PATH: &'static str = "models/containers";
}

impl Create<ContainerCreateDefinition, ContainerDefinition> for Containers {}
impl List<ContainerListQuery, ContainerDefinition> for Containers {}
impl Retrieve<ContainerReference, ContainerDefinition> for Containers {}
impl RetrieveWithQuery<ContainerReference, ContainerRetrieveQuery, ContainerDefinition>
    for Containers
{
}
impl DeleteWithResponse<ContainerReference, ContainerReference> for Containers {}
