use std::fmt::Display;

use crate::Partition;

/// Trait for query parameters.
pub trait IntoParams {
    /// Convert self to a list of query parameter tuples.
    fn into_params(self) -> Vec<(String, String)>;
}

impl IntoParams for Vec<(String, String)> {
    fn into_params(self) -> Vec<(String, String)> {
        self
    }
}

/// Push the item given in `item` to the query with name `name` if it is Some.
pub fn to_query<T>(name: &str, item: &Option<T>, params: &mut Vec<(String, String)>)
where
    T: Display,
{
    match item {
        Some(it) => params.push((name.to_string(), it.to_string())),
        None => (),
    }
}

/// Push a list of items to the query with name `name` if `item` is `Some`.
pub fn to_query_vec(name: &str, item: &Option<Vec<String>>, params: &mut Vec<(String, String)>) {
    match item {
        Some(it) => params.push((
            name.to_string(),
            format!(
                "[{}]",
                it.iter()
                    .map(|val| { format!("\"{val}\"") })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        )),
        None => (),
    }
}

/// Push a list of numbers to the query with name `name` if `item` is `Some`.
pub fn to_query_vec_i64(name: &str, item: &Option<Vec<i64>>, params: &mut Vec<(String, String)>) {
    match item {
        Some(it) => params.push((
            name.to_string(),
            format!(
                "[{}]",
                it.iter()
                    .map(|val| { format!("{val}") })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        )),
        None => (),
    }
}

/// Simple query with limit and cursor.
#[derive(Debug, Default, Clone)]
pub struct LimitCursorQuery {
    /// Maximum number of results to return.
    pub limit: Option<i32>,
    /// Cursor for pagination.
    pub cursor: Option<String>,
}

impl IntoParams for LimitCursorQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("limit", &self.limit, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        params
    }
}

/// Query with limt, cursor, and partition.
#[derive(Debug, Default, Clone)]
pub struct LimitCursorPartitionQuery {
    /// Maximum number of results to return.
    pub limit: Option<i32>,
    /// Cursor for pagination.
    pub cursor: Option<String>,
    /// Partition count and number.
    pub partition: Option<Partition>,
}

impl IntoParams for LimitCursorPartitionQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("limit", &self.limit, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("partition", &self.partition, &mut params);
        params
    }
}
