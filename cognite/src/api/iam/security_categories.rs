use crate::api::resource::Resource;
use crate::dto::iam::security_category::*;
use crate::{Create, Delete, List, WithBasePath};

/// Manage security categories for a specific project. Security categories can be used to
/// restrict access to a resource. Applying a security category to a resource means that
/// only principals (users or service accounts) that also have this security category
/// can access the resource.
pub type SecurityCategoriesResource = Resource<SecurityCategory>;

impl WithBasePath for SecurityCategoriesResource {
    const BASE_PATH: &'static str = "securitycategories";
}

impl Create<AddSecurityCategory, SecurityCategory> for SecurityCategoriesResource {}
impl List<SecurityCategoryQuery, SecurityCategory> for SecurityCategoriesResource {}
impl Delete<u64> for SecurityCategoriesResource {}
