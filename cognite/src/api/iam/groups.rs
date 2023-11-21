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

impl Create<AddGroup, Group> for GroupsResource {}
impl List<GroupQuery, Group> for GroupsResource {}
impl Delete<u64> for GroupsResource {}
