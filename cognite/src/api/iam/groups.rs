use crate::api::resource::*;
use crate::dto::iam::group::*;
use crate::Create;
use crate::WithBasePath;

pub type Groups = Resource<Group>;

impl WithBasePath for Groups {
    const BASE_PATH: &'static str = "groups";
}

impl Create<AddGroup, Group> for Groups {}
impl List<GroupQuery, Group> for Groups {}
impl Delete<u64> for Groups {}
