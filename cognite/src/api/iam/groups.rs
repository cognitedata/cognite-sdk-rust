use crate::api::resource::*;
use crate::dto::iam::group::*;
use crate::Create;
use crate::WithBasePath;

/// Groups are used to give principals the capabilities to access CDF resources. One principal
/// can be a member of multiple groups, and one group can have multiple members.
pub type GroupsResource = Resource<Group>;

impl WithBasePath for GroupsResource {
    const BASE_PATH: &'static str = "groups";
}

impl WithChunkSizes for GroupsResource {
    const REQUEST_CHUNK_SIZE: usize = 100;
    const REQUEST_PARALLELISM: usize = 4;
}

impl Create<AddGroup, Group> for GroupsResource {}
impl List<GroupQuery, Group> for GroupsResource {}
impl Delete<u64> for GroupsResource {}
