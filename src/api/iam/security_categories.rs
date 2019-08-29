use crate::api::ApiClient;
use crate::dto::iam::security_category::*;
use crate::dto::items::Items;
use crate::dto::params::Params;
use crate::error::Result;

pub struct SecurityCategories {
    api_client: ApiClient,
}

impl SecurityCategories {
    pub fn new(api_client: ApiClient) -> SecurityCategories {
        SecurityCategories {
            api_client: api_client,
        }
    }

    pub fn list_all(&self, params: Option<Vec<Params>>) -> Result<Vec<SecurityCategory>> {
        let security_categories_response: SecurityCategoryListResponse = self
            .api_client
            .get_with_params("securitycategories", params)?;
        Ok(security_categories_response.items)
    }

    pub fn create(
        &self,
        security_category_names: &[SecurityCategory],
    ) -> Result<Vec<SecurityCategory>> {
        let security_category_name_items = Items::from(security_category_names);
        let security_categories_response: SecurityCategoryListResponse = self
            .api_client
            .post("securitycategories", &security_category_name_items)?;
        Ok(security_categories_response.items)
    }

    pub fn delete(&self, security_category_ids: &[u64]) -> Result<()> {
        let id_items = Items::from(security_category_ids);
        self.api_client
            .post::<::serde_json::Value, Items>("securitycategories/delete", &id_items)?;
        Ok(())
    }
}
