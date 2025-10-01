use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{EqIdentity, Identity};

/// Options for performing automatic upserts.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct UpsertOptions {
    /// Ignore null values of fields when updating. If this is `false` (the default),
    /// missing values will be set to `null`.
    pub ignore_nulls: bool,
    /// Merge maps when updating. If this is `false` (the default),
    /// maps (like metadata) will be overwritten.
    pub merge_maps: bool,
    /// Merge lists when updating. If this is `false` (the default),
    /// lists will be overwritten.
    pub merge_lists: bool,
}

impl UpsertOptions {
    /// Ignore null values of fields when updating. If this is `false` (the default),
    /// missing values will be set to `null`.
    pub fn ignore_nulls(mut self, ignore_nulls: bool) -> Self {
        self.ignore_nulls = ignore_nulls;
        self
    }

    /// Merge metadata maps when updating. If this is `false` (the default),
    /// metadata will be overwritten.
    pub fn merge_metadata(mut self, merge_metadata: bool) -> Self {
        self.merge_maps = merge_metadata;
        self
    }

    /// Merge lists when updating. If this is `false` (the default),
    /// lists will be overwritten.
    pub fn merge_lists(mut self, merge_lists: bool) -> Self {
        self.merge_lists = merge_lists;
        self
    }
}

/// Trait for converting a value into a patch item, used for upsert.
pub trait IntoPatchItem<TPatch> {
    /// Convert self into a patch, optionally ignoring null values.
    fn patch(self, options: &UpsertOptions) -> Option<TPatch>;
}

/// Trait for converting a value into a patch, used for upsert.
pub trait IntoPatch<TPatch> {
    /// Convert self into a patch, optionally ignoring null values.
    fn patch(self, options: &UpsertOptions) -> TPatch;
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged, rename_all_fields = "camelCase")]
/// Update the value of an item, or set it to null.
pub enum UpdateSetNull<T> {
    /// Set a new value.
    Set {
        /// New value to set.
        set: T,
    },
    /// Set the value to null.
    SetNull {
        /// Whether to set the value to null, or leave it unmodified.
        set_null: bool,
    },
}

impl<T> Default for UpdateSetNull<T> {
    fn default() -> Self {
        Self::SetNull { set_null: false }
    }
}

impl<T> IntoPatchItem<UpdateSetNull<T>> for Option<T> {
    fn patch(self, options: &UpsertOptions) -> Option<UpdateSetNull<T>> {
        match (self, options.ignore_nulls) {
            (None, true) => None,
            (None, false) => Some(UpdateSetNull::SetNull { set_null: true }),
            (Some(x), _) => Some(UpdateSetNull::Set { set: x }),
        }
    }
}

impl<T> UpdateSetNull<T> {
    /// Set a new value
    ///
    /// # Arguments
    ///
    /// * `value` - Value to set.
    pub fn set(value: T) -> Self {
        Self::Set { set: value }
    }

    /// Set the value to null.
    ///
    /// # Arguments
    ///
    /// * `set_null` - Whether to set the value to null, or leave it unmodified.
    pub fn set_null(set_null: bool) -> Self {
        Self::SetNull { set_null }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Update the value of an item.
pub struct UpdateSet<T> {
    /// New value of item.
    pub set: T,
}

impl<T> UpdateSet<T> {
    /// Set a new value
    ///
    /// # Arguments
    ///
    /// * `value` - Value to set.
    pub fn set(value: T) -> Self {
        Self { set: value }
    }
}

impl<T> IntoPatchItem<UpdateSet<T>> for T {
    fn patch(self, _options: &UpsertOptions) -> Option<UpdateSet<T>> {
        Some(UpdateSet { set: self })
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged, rename_all_fields = "camelCase")]
/// Update the value of a list item, adding and removing or setting the values.
pub enum UpdateList<TAdd, TRemove> {
    /// Add new values and remove old values.
    AddRemove {
        /// New values to add.
        add: Option<Vec<TAdd>>,
        /// Old values to remove.
        remove: Option<Vec<TRemove>>,
    },
    /// Set the list to a new value.
    Set {
        /// New value for list.
        set: Vec<TAdd>,
    },
}

impl<TAdd, TRemove> UpdateList<TAdd, TRemove> {
    /// Add items given by `add` and remove any given by `remove`
    ///
    /// # Arguments
    ///
    /// * `add` - New values to add.
    /// * `remove` - Old values to remove.
    pub fn add_remove(add: Vec<TAdd>, remove: Vec<TRemove>) -> Self {
        Self::AddRemove {
            add: Some(add),
            remove: Some(remove),
        }
    }

    /// Add items given by `add`, overwriting any that already exist.
    ///
    /// # Arguments
    ///
    /// * `add` - New values to add.
    pub fn add(add: Vec<TAdd>) -> Self {
        Self::AddRemove {
            add: Some(add),
            remove: None,
        }
    }

    /// Remove items given by `remove`, if they exist.
    ///
    /// # Arguments
    ///
    /// * `remove` - Old values to remove.
    pub fn remove(remove: Vec<TRemove>) -> Self {
        Self::AddRemove {
            add: None,
            remove: Some(remove),
        }
    }

    /// Set the list to `set`.
    ///
    /// # Arguments
    ///
    /// * `set` - New value for list.
    pub fn set(set: Vec<TAdd>) -> Self {
        Self::Set { set }
    }
}

impl<TAdd, TRemove> IntoPatchItem<UpdateList<TAdd, TRemove>> for Vec<TAdd> {
    fn patch(self, options: &UpsertOptions) -> Option<UpdateList<TAdd, TRemove>> {
        if options.merge_lists {
            Some(UpdateList::add(self))
        } else {
            Some(UpdateList::set(self))
        }
    }
}

impl<TAdd, TRemove> IntoPatchItem<UpdateList<TAdd, TRemove>> for Option<Vec<TAdd>> {
    fn patch(self, options: &UpsertOptions) -> Option<UpdateList<TAdd, TRemove>> {
        match (self, options.ignore_nulls, options.merge_lists) {
            (Some(x), _, true) => Some(UpdateList::add(x)),
            (Some(x), _, false) => Some(UpdateList::set(x)),
            (None, false, false) => Some(UpdateList::set(vec![])),
            (None, _, _) => None,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged, rename_all_fields = "camelCase")]
/// Update a map from `TKey` to `TValue`, adding and removing or setting the values.
pub enum UpdateMap<TKey, TValue>
where
    TKey: std::hash::Hash + std::cmp::Eq,
{
    /// Add new values and remove existing keys.
    AddRemove {
        /// New values to add, any existing will be overwritten.
        add: Option<HashMap<TKey, TValue>>,
        /// Old keys to remove.
        remove: Option<Vec<TKey>>,
    },
    /// Set the map to a new value.
    Set {
        /// New value for map.
        set: HashMap<TKey, TValue>,
    },
}

impl<TKey, TValue> UpdateMap<TKey, TValue>
where
    TKey: std::hash::Hash + std::cmp::Eq,
{
    /// Add items given by `add` and remove any given by `remove`
    ///
    /// # Arguments
    ///
    /// * `add` - New values to add, any existing will be overwritten.
    /// * `remove` - Old keys to remove.
    pub fn add_remove(add: HashMap<TKey, TValue>, remove: Vec<TKey>) -> Self {
        Self::AddRemove {
            add: Some(add),
            remove: Some(remove),
        }
    }

    /// Add items given by `add`, overwriting any that already exist.
    ///
    /// # Arguments
    ///
    /// * `add` - New values to add, any existing will be overwritten.
    pub fn add(add: HashMap<TKey, TValue>) -> Self {
        Self::AddRemove {
            add: Some(add),
            remove: None,
        }
    }

    /// Remove items given by `remove`, if they exist.
    ///
    /// # Arguments
    ///
    /// * `remove` - Old keys to remove.
    pub fn remove(remove: Vec<TKey>) -> Self {
        Self::AddRemove {
            add: None,
            remove: Some(remove),
        }
    }

    /// Set the map to `set`.
    ///
    /// # Arguments
    ///
    /// * `set` - New value for map.
    pub fn set(set: HashMap<TKey, TValue>) -> Self {
        Self::Set { set }
    }
}

impl<TKey: std::hash::Hash + std::cmp::Eq, TValue> IntoPatchItem<UpdateMap<TKey, TValue>>
    for HashMap<TKey, TValue>
{
    fn patch(self, options: &UpsertOptions) -> Option<UpdateMap<TKey, TValue>> {
        if options.merge_maps {
            Some(UpdateMap::add(self))
        } else {
            Some(UpdateMap::set(self))
        }
    }
}

impl<TKey: std::hash::Hash + std::cmp::Eq, TValue> IntoPatchItem<UpdateMap<TKey, TValue>>
    for Option<HashMap<TKey, TValue>>
{
    fn patch(self, options: &UpsertOptions) -> Option<UpdateMap<TKey, TValue>> {
        match (self, options.ignore_nulls, options.merge_lists) {
            (Some(x), _, true) => Some(UpdateMap::add(x)),
            (Some(x), _, false) => Some(UpdateMap::set(x)),
            (None, false, false) => Some(UpdateMap::set(HashMap::new())),
            (None, _, _) => None,
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
    /// Identity of resource to update.
    pub id: Identity,
    /// Resource patch.
    pub update: T,
}

impl<T> Patch<T>
where
    T: Default,
{
    /// Create a new patch
    ///
    /// # Arguments
    ///
    /// * `id` - Identity of resource to update.
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
