use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{EqIdentity, Identity};

pub trait IntoPatchItem<TPatch> {
    fn patch(self, ignore_nulls: bool) -> Option<TPatch>;
}

pub trait IntoPatch<TPatch> {
    fn patch(self, ignore_nulls: bool) -> TPatch;
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", untagged)]
/// Update the value of an item, or set it to null.
pub enum UpdateSetNull<T> {
    Set {
        set: T,
    },
    #[serde(rename_all = "camelCase")]
    SetNull {
        set_null: bool,
    },
}

impl<T> Default for UpdateSetNull<T> {
    fn default() -> Self {
        Self::SetNull { set_null: false }
    }
}

impl<T> IntoPatchItem<UpdateSetNull<T>> for Option<T> {
    fn patch(self, ignore_nulls: bool) -> Option<UpdateSetNull<T>> {
        match (self, ignore_nulls) {
            (None, true) => None,
            (None, false) => Some(UpdateSetNull::SetNull { set_null: true }),
            (Some(x), _) => Some(UpdateSetNull::Set { set: x }),
        }
    }
}

impl<T> UpdateSetNull<T> {
    pub fn set(value: T) -> Self {
        Self::Set { set: value }
    }

    pub fn set_null(set_null: bool) -> Self {
        Self::SetNull { set_null }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Update the value of an item.
pub struct UpdateSet<T> {
    pub set: T,
}

impl<T> UpdateSet<T> {
    pub fn set(value: T) -> Self {
        Self { set: value }
    }
}

impl<T> IntoPatchItem<UpdateSet<T>> for T {
    fn patch(self, _ignore_nulls: bool) -> Option<UpdateSet<T>> {
        Some(UpdateSet { set: self })
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", untagged)]
/// Update the value of a list item, adding and removing or setting the values.
pub enum UpdateList<TAdd, TRemove> {
    AddRemove {
        add: Option<Vec<TAdd>>,
        remove: Option<Vec<TRemove>>,
    },
    Set {
        set: Vec<TAdd>,
    },
}

impl<TAdd, TRemove> UpdateList<TAdd, TRemove> {
    /// Add items given by `add` and remove any given by `remove`
    pub fn add_remove(add: Vec<TAdd>, remove: Vec<TRemove>) -> Self {
        Self::AddRemove {
            add: Some(add),
            remove: Some(remove),
        }
    }

    /// Add items given by `add`, overwriting any that already exist.
    pub fn add(add: Vec<TAdd>) -> Self {
        Self::AddRemove {
            add: Some(add),
            remove: None,
        }
    }

    /// Remove items given by `remove`, if they exist.
    pub fn remove(remove: Vec<TRemove>) -> Self {
        Self::AddRemove {
            add: None,
            remove: Some(remove),
        }
    }

    /// Set the list to `set`.
    pub fn set(set: Vec<TAdd>) -> Self {
        Self::Set { set }
    }
}

impl<TAdd, TRemove> IntoPatchItem<UpdateList<TAdd, TRemove>> for Vec<TAdd> {
    fn patch(self, _ignore_nulls: bool) -> Option<UpdateList<TAdd, TRemove>> {
        Some(UpdateList::set(self))
    }
}

impl<TAdd, TRemove> IntoPatchItem<UpdateList<TAdd, TRemove>> for Option<Vec<TAdd>> {
    fn patch(self, ignore_nulls: bool) -> Option<UpdateList<TAdd, TRemove>> {
        match (self, ignore_nulls) {
            (Some(x), _) => Some(UpdateList::set(x)),
            (None, true) => None,
            (None, false) => Some(UpdateList::set(vec![])),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", untagged)]
/// Update a map from `TKey` to `TValue`, adding and removing or setting the values.
pub enum UpdateMap<TKey, TValue>
where
    TKey: std::hash::Hash + std::cmp::Eq,
{
    AddRemove {
        add: Option<HashMap<TKey, TValue>>,
        remove: Option<Vec<TKey>>,
    },
    Set {
        set: HashMap<TKey, TValue>,
    },
}

impl<TKey, TValue> UpdateMap<TKey, TValue>
where
    TKey: std::hash::Hash + std::cmp::Eq,
{
    /// Add items given by `add` and remove any given by `remove`
    pub fn add_remove(add: HashMap<TKey, TValue>, remove: Vec<TKey>) -> Self {
        Self::AddRemove {
            add: Some(add),
            remove: Some(remove),
        }
    }

    /// Add items given by `add`, overwriting any that already exist.
    pub fn add(add: HashMap<TKey, TValue>) -> Self {
        Self::AddRemove {
            add: Some(add),
            remove: None,
        }
    }

    /// Remove items given by `remove`, if they exist.
    pub fn remove(remove: Vec<TKey>) -> Self {
        Self::AddRemove {
            add: None,
            remove: Some(remove),
        }
    }

    /// Set the list to `set`.
    pub fn set(set: HashMap<TKey, TValue>) -> Self {
        Self::Set { set }
    }
}

impl<TKey: std::hash::Hash + std::cmp::Eq, TValue> IntoPatchItem<UpdateMap<TKey, TValue>>
    for HashMap<TKey, TValue>
{
    fn patch(self, _ignore_nulls: bool) -> Option<UpdateMap<TKey, TValue>> {
        Some(UpdateMap::set(self))
    }
}

impl<TKey: std::hash::Hash + std::cmp::Eq, TValue> IntoPatchItem<UpdateMap<TKey, TValue>>
    for Option<HashMap<TKey, TValue>>
{
    fn patch(self, ignore_nulls: bool) -> Option<UpdateMap<TKey, TValue>> {
        match (self, ignore_nulls) {
            (None, true) => None,
            (None, false) => Some(UpdateMap::set(HashMap::new())),
            (Some(x), _) => Some(UpdateMap::set(x)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Wrapper around a patch update.
pub struct Patch<T>
where
    T: Default,
{
    #[serde(flatten)]
    pub id: Identity,
    pub update: T,
}

impl<T> Patch<T>
where
    T: Default,
{
    pub fn new(id: Identity) -> Self {
        Patch::<T> {
            id,
            update: T::default(),
        }
    }
}

impl<T> EqIdentity for Patch<T>
where
    T: Default,
{
    fn eq(&self, id: &Identity) -> bool {
        &self.id == id
    }
}

/// Macro to extract the identity from a resource.
macro_rules! to_idt {
    ($it:ident) => {
        if $it.id > 0 {
            Identity::Id { id: $it.id }
        } else {
            $it.external_id
                .as_ref()
                .map(|e| Identity::ExternalId {
                    external_id: e.clone(),
                })
                .unwrap_or_else(|| Identity::Id { id: $it.id })
        }
    };
}
