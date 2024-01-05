use crate::{
    dto::data_modeling::views::{ViewDefinition, ViewQuery},
    models::{
        views::{ViewCreateDefinition, ViewReference},
        ItemIdOptionalVersion,
    },
    Create, DeleteWithResponse, List, Resource, Retrieve, WithBasePath,
};

/// A view is a logical view on the stored data. Views organize
/// and describe properties defined in various containers, making them easier to query.
pub type ViewsResource = Resource<ViewDefinition>;

impl WithBasePath for ViewsResource {
    const BASE_PATH: &'static str = "models/views";
}

impl Create<ViewCreateDefinition, ViewDefinition> for ViewsResource {}
impl List<ViewQuery, ViewDefinition> for ViewsResource {}
impl Retrieve<ItemIdOptionalVersion, ViewDefinition> for ViewsResource {}
impl DeleteWithResponse<ViewReference, ViewReference> for ViewsResource {}
