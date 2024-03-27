use serde::{Deserialize, Serialize};

use crate::{models::SortDirection, to_query, IntoParams, SetCursor};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Create a security category.
pub struct AddSecurityCategory {
    /// Security category name.
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// CDF security category.
pub struct SecurityCategory {
    /// Security category name.
    pub name: String,
    /// Internal ID.
    pub id: u64,
}

#[derive(Debug, Default)]
/// Filter security categories.
pub struct SecurityCategoryQuery {
    /// Sort security categories in ascending or descending order.
    pub sort: Option<SortDirection>,
    /// Cursor for pagination.
    pub cursor: Option<String>,
    /// Optional limit. Default is 25, maximum is 1000.
    pub limit: Option<i32>,
}

impl IntoParams for SecurityCategoryQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query(
            "sort",
            &self.sort.as_ref().map(|f| match f {
                SortDirection::Ascending => "ASC",
                SortDirection::Descending => "DESC",
            }),
            &mut params,
        );
        to_query("cursor", &self.cursor, &mut params);
        to_query("limit", &self.limit, &mut params);
        params
    }
}

impl SetCursor for SecurityCategoryQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}
