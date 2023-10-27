use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{to_query, AsParams};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecurityCategoryListResponse {
    pub items: Vec<SecurityCategory>,
    previous_cursor: Option<String>,
    next_cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SecurityCategory {
    pub name: String,
    pub id: Option<u64>,
}

#[derive(Debug, Default)]
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
