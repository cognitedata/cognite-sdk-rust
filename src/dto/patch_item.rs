use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchItem {
    #[serde(skip_serializing_if = "::serde_json::Value::is_null")]
    pub set: ::serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_null: Option<bool>,
}

impl PatchItem {
    pub fn convert_option<T: Serialize>(item: &Option<T>) -> Option<PatchItem> {
        match item {
            Some(i) => Some(PatchItem::from(i)),
            None => None,
        }
    }

    pub fn convert<T: Serialize>(item: &T) -> Option<PatchItem> {
        Some(PatchItem::from(item))
    }
}

impl<T: Serialize> From<&T> for PatchItem {
    fn from(item: &T) -> PatchItem {
        PatchItem {
            set: json!(item),
            set_null: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<u64>>,
}

impl PatchList {
    pub fn convert(list: &Option<Vec<u64>>) -> Option<PatchList> {
        match list {
            Some(l) => {
                if l.is_empty() {
                    None
                } else {
                    Some(PatchList::from(l))
                }
            }
            None => None,
        }
    }
}

impl From<&[u64]> for PatchList {
    fn from(items: &[u64]) -> PatchList {
        PatchList {
            add: None,
            remove: None,
            set: Some(items.to_vec()),
        }
    }
}

impl From<&Vec<u64>> for PatchList {
    fn from(items: &Vec<u64>) -> PatchList {
        PatchList {
            add: None,
            remove: None,
            set: Some(items.clone()),
        }
    }
}
