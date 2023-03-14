use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{to_query, AsParams};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecurityCategoryListResponse {
    pub items: Vec<SecurityCategory>,
    previous_cursor: Option<String>,
    next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecurityCategory {
    pub name: String,
    pub id: Option<u64>,
}

#[derive(Debug)]
#[derive(Default)]
pub enum SecurityCategorySortEnum {
    #[default]
    ASC,
    DESC,
}



impl Display for SecurityCategorySortEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityCategorySortEnum::ASC => write!(f, "ASC"),
            SecurityCategorySortEnum::DESC => write!(f, "DESC"),
        }
    }
}

#[derive(Debug, Default)]
pub struct SecurityCategoryQuery {
    pub sort: Option<SecurityCategorySortEnum>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

impl AsParams for SecurityCategoryQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("sort", &self.sort, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("limit", &self.limit, &mut params);
        params
    }
}
