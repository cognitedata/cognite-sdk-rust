use crate::{
    models::{
        spaces::{Space, SpaceCreate},
        SpaceId,
    },
    Create, DeleteWithResponse, LimitCursorQuery, List, Resource, Retrieve, WithBasePath,
};

/// Spaces contain and namespace instances, views, and containers.
/// External IDs must be unique only within a space.
pub type SpacesResource = Resource<Space>;

impl WithBasePath for SpacesResource {
    const BASE_PATH: &'static str = "models/spaces";
}

impl Create<SpaceCreate, Space> for SpacesResource {}
impl List<LimitCursorQuery, Space> for SpacesResource {}
impl Retrieve<SpaceId, Space> for SpacesResource {}
impl DeleteWithResponse<SpaceId, SpaceId> for SpacesResource {}
