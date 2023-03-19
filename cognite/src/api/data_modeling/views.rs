use crate::{
    dto::data_modeling::views::{ViewDefinition, ViewQuery},
    models::{ItemIdWithOptionalVersion, ViewCreateDefinition, ViewReference},
    Create, DeleteWithResponse, List, Resource, Retrieve, WithBasePath,
};

pub struct View {}
pub type Views = Resource<View>;

impl WithBasePath for Views {
    const BASE_PATH: &'static str = "models/views";
}

impl Create<ViewCreateDefinition, ViewDefinition> for Views {}
impl List<ViewQuery, ViewDefinition> for Views {}
impl Retrieve<ItemIdWithOptionalVersion, ViewDefinition> for Views {}
impl DeleteWithResponse<ViewReference, ViewReference> for Views {}
