use crate::api::resource::Resource;
use crate::dto::iam::security_category::*;
use crate::dto::items::Items;
use crate::error::Result;

/// Manage security categories for a specific project. Security categories can be used to
/// restrict access to a resource. Applying a security category to a resource means that
/// only principals (users or service accounts) that also have this security category
/// can access the resource.
pub type SecurityCategoriesResource = Resource<SecurityCategory>;

impl SecurityCategoriesResource {
    pub async fn list_all(
        &self,
        params: Option<SecurityCategoryQuery>,
    ) -> Result<Vec<SecurityCategory>> {
        let security_categories_response: SecurityCategoryListResponse = self
            .api_client
            .get_with_params("securitycategories", params)
            .await?;
        Ok(security_categories_response.items)
    }

    pub async fn create(
        &self,
        security_category_names: &[SecurityCategory],
    ) -> Result<Vec<SecurityCategory>> {
        let security_category_name_items = Items::from(security_category_names);
        let security_categories_response: SecurityCategoryListResponse = self
            .api_client
            .post("securitycategories", &security_category_name_items)
            .await?;
        Ok(security_categories_response.items)
    }

    pub async fn delete(&self, security_category_ids: &[u64]) -> Result<()> {
        let id_items = Items::from(security_category_ids);
        self.api_client
            .post::<::serde_json::Value, Items>("securitycategories/delete", &id_items)
            .await?;
        Ok(())
    }
}
