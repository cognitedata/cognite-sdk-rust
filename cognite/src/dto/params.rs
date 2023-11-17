use std::fmt::Display;

use crate::Partition;

/// Trait for query parameters.
pub trait AsParams {
    /// Convert self to a list of query parameter tuples.
    fn to_tuples(self) -> Vec<(String, String)>;
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
pub struct LimitCursorQuery {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

impl AsParams for LimitCursorQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("limit", &self.limit, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        params
    }
}

/// Query with limt, cursor, and partition.
pub struct LimitCursorPartitionQuery {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
    pub partition: Option<Partition>,
}

impl AsParams for LimitCursorPartitionQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("limit", &self.limit, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("partition", &self.partition, &mut params);
        params
    }
}
