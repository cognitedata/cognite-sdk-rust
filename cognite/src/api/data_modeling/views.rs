use crate::{
    dto::data_modeling::views::{ViewDefinition, ViewQuery},
    models::{ItemIdOptionalVersion, ItemIdWithVersion, ViewCreateDefinition},
    Create, DeleteWithResponse, List, Resource, Retrieve, WithBasePath,
};

pub struct View {}
pub type Views = Resource<View>;

impl WithBasePath for Views {
    const BASE_PATH: &'static str = "models/views";
}

impl Create<ViewCreateDefinition, ViewDefinition> for Views {}
impl List<ViewQuery, ViewDefinition> for Views {}
impl Retrieve<ItemIdOptionalVersion, ViewDefinition> for Views {}
impl DeleteWithResponse<ItemIdWithVersion, ItemIdWithVersion> for Views {}
