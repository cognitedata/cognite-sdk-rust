use crate::{
    models::{Space, SpaceCreate, SpaceId},
    Create, DeleteWithResponse, LimitCursorQuery, List, Resource, Retrieve, WithBasePath,
};

pub struct SpaceResource {}
pub type Spaces = Resource<SpaceResource>;

impl WithBasePath for Spaces {
    const BASE_PATH: &'static str = "models/spaces";
}

impl Create<SpaceCreate, Space> for Spaces {}
impl List<LimitCursorQuery, Space> for Spaces {}
impl Retrieve<SpaceId, Space> for Spaces {}
impl DeleteWithResponse<SpaceId, SpaceId> for Spaces {}
