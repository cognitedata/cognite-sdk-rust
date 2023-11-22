use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A wrapper around a list of items.
pub struct Items<T> {
    pub items: T,
}

impl<'a, T: Serialize> From<&'a Vec<T>> for Items<&'a Vec<T>> {
    fn from(items: &'a Vec<T>) -> Self {
        Items { items }
    }
}

impl<'a, T: Serialize> From<Vec<&'a T>> for Items<Vec<&'a T>> {
    fn from(items: Vec<&'a T>) -> Self {
        Items { items }
    }
}

impl<'a, T: Serialize> From<&'a [T]> for Items<&'a [T]> {
    fn from(items: &'a [T]) -> Self {
        Items { items }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A wrapper around a list of items, with cursor.
pub struct ItemsWithCursor<T>
where
    T: Serialize,
{
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A wrapper around a list of items, without cursor.
pub struct ItemsWithoutCursor<T>
where
    T: Serialize,
{
    pub items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A wrapper around a list of items, with ignore unknown ids.
pub struct ItemsWithIgnoreUnknownIds<T> {
    pub items: T,
    pub ignore_unknown_ids: bool,
}

impl<T> ItemsWithIgnoreUnknownIds<T> {
    pub fn new(items: T, ignore_unknown_ids: bool) -> Self
    where
        T: Serialize,
    {
        Self {
            items,
            ignore_unknown_ids,
        }
    }
}

impl<'a, T: Serialize> From<&'a Vec<T>> for ItemsWithIgnoreUnknownIds<&'a Vec<T>> {
    fn from(items: &'a Vec<T>) -> Self {
        ItemsWithIgnoreUnknownIds {
            items,
            ignore_unknown_ids: true,
        }
    }
}

impl<'a, T: Serialize> From<&'a [T]> for ItemsWithIgnoreUnknownIds<&'a [T]> {
    fn from(items: &'a [T]) -> Self {
        ItemsWithIgnoreUnknownIds {
            items,
            ignore_unknown_ids: true,
        }
    }
}
