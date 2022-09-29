use std::fmt::Display;

pub trait AsParams {
    fn to_tuples(self) -> Vec<(String, String)>;
}

pub fn to_query<T>(name: &str, item: &Option<T>, params: &mut Vec<(String, String)>)
where
    T: Display,
{
    match item {
        Some(it) => params.push((name.to_string(), it.to_string())),
        None => (),
    }
}

pub fn to_query_vec(name: &str, item: &Option<Vec<String>>, params: &mut Vec<(String, String)>) {
    match item {
        Some(it) => params.push((
            name.to_string(),
            format!(
                "[{}]",
                it.iter()
                    .map(|val| { format!("\"{}\"", val) })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        )),
        None => (),
    }
}

pub fn to_query_vec_i64(name: &str, item: &Option<Vec<i64>>, params: &mut Vec<(String, String)>) {
    match item {
        Some(it) => params.push((
            name.to_string(),
            format!(
                "[{}]",
                it.iter()
                    .map(|val| { format!("{}", val) })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        )),
        None => (),
    }
}

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

pub struct LimitCursorPartitionQuery {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
    pub partition: Option<String>,
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
