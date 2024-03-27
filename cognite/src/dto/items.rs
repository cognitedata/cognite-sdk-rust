use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A generic structure for handling items in requests and responses.
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items<C, E = ()> {
    /// The items of the request or response.
    pub items: C,
    /// Additional fields, delegated to the `E` type.
    #[serde(flatten)]
    pub extra_fields: E,
}

impl<C> Items<C> {
    /// Create a new `Items` instance with the provided items and no extra fields.
    pub fn new(items: C) -> Self {
        Self {
            items,
            extra_fields: (),
        }
    }
}

impl<C, E> Items<C, E> {
    /// Create a new `Items` instance with the provided items and extra fields.
    pub fn new_with_extra_fields(items: C, extra_fields: E) -> Self {
        Self {
            items,
            extra_fields,
        }
    }
}

impl<C, E: Default> From<C> for Items<C, E> {
    fn from(items: C) -> Self {
        Items {
            items,
            extra_fields: E::default(),
        }
    }
}

/// A convenience type for `Items` using a `Vec` to store items.
pub type ItemsVec<T, E = ()> = Items<Vec<T>, E>;

/// Extra fields for `Items` types with cursor data.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Cursor {
    /// The cursor for the next page of items.
    pub next_cursor: Option<String>,
}

/// Extra fields for `Items` types with the `ignoreUnknownIds` field.
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct IgnoreUnknownIds {
    /// Whether to ignore unknown IDs in the request.
    pub ignore_unknown_ids: bool,
}
