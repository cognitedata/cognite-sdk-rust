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

impl<T: Serialize> From<&[T]> for Items {
    fn from(items: &[T]) -> Items {
        Items {
            items: json!(items),
        }
    }
}
