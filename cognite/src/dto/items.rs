use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Items {
    pub items: ::serde_json::Value,
}

impl<T: Serialize> From<&Vec<T>> for Items {
    fn from(items: &Vec<T>) -> Items {
        Items {
            items: json!(items),
        }
    }
}

impl<T: Serialize> From<Vec<&T>> for Items {
    fn from(items: Vec<&T>) -> Items {
        Items {
            items: json!(items),
        }
    }
}

impl<T: Serialize> From<&[T]> for Items {
    fn from(items: &[T]) -> Items {
        Items {
            items: json!(items),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemsWithCursor<T>
where
    T: Serialize,
{
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemsWithoutCursor<T>
where
    T: Serialize,
{
    pub items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemsWithIgnoreUnknownIds {
    pub items: ::serde_json::Value,
    pub ignore_unknown_ids: bool,
}

impl ItemsWithIgnoreUnknownIds {
    pub fn new<T>(items: &[T], ignore_unknown_ids: bool) -> Self
    where
        T: Serialize,
    {
        Self {
            items: json!(items),
            ignore_unknown_ids,
        }
    }
}

impl<T: Serialize> From<&Vec<T>> for ItemsWithIgnoreUnknownIds {
    fn from(items: &Vec<T>) -> ItemsWithIgnoreUnknownIds {
        ItemsWithIgnoreUnknownIds {
            items: json!(items),
            ignore_unknown_ids: true,
        }
    }
}

impl<T: Serialize> From<&[T]> for ItemsWithIgnoreUnknownIds {
    fn from(items: &[T]) -> ItemsWithIgnoreUnknownIds {
        ItemsWithIgnoreUnknownIds {
            items: json!(items),
            ignore_unknown_ids: true,
        }
    }
}