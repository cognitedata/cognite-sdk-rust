use serde::{Deserialize, Serialize};

use crate::{models::SortDirection, to_query, AsParams, SetCursor};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddSecurityCategory {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SecurityCategory {
    pub name: String,
    pub id: u64,
}

#[derive(Debug, Default)]
pub struct SecurityCategoryQuery {
    pub sort: Option<SortDirection>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

impl AsParams for SecurityCategoryQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
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
