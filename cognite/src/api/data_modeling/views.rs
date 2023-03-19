use crate::{
    dto::data_modeling::{
        common::ViewReference,
        views::{ViewDefinition, ViewQuery},
    },
    DeleteWithResponse, List, Resource, WithBasePath,
};

pub struct View {}
pub type Views = Resource<View>;

impl WithBasePath for Views {
    const BASE_PATH: &'static str = "models/views";
}

impl List<ViewQuery, ViewDefinition> for Views {}
impl DeleteWithResponse<ViewReference, ViewReference> for Views {}
