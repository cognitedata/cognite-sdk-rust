use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{EqIdentity, Identity};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSetNull<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_null: Option<bool>,
}

impl<T> From<Option<T>> for UpdateSetNull<T> {
    fn from(el: Option<T>) -> Self {
        match el {
            Some(x) => UpdateSetNull {
                set: Some(x),
                set_null: None,
            },
            None => UpdateSetNull {
                set: None,
                set_null: Some(true),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSet<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<T>,
}

impl<T> From<T> for UpdateSet<T> {
    fn from(el: T) -> Self {
        UpdateSet { set: Some(el) }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateList<TAdd, TRemove> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<TAdd>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<TRemove>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<TAdd>>,
}

impl<TAdd, TRemove> From<Vec<TAdd>> for UpdateList<TAdd, TRemove> {
    fn from(el: Vec<TAdd>) -> Self {
        UpdateList::<TAdd, TRemove> {
            set: Some(el),
            remove: None,
            add: None,
        }
    }
}

impl<TAdd, TRemove> From<Option<Vec<TAdd>>> for UpdateList<TAdd, TRemove> {
    fn from(el: Option<Vec<TAdd>>) -> Self {
        match el {
            Some(x) => x.into(),
            None => UpdateList::<TAdd, TRemove> {
                set: Some(Vec::<TAdd>::new()),
                remove: None,
                add: None,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMap<TKey, TValue>
where
    TKey: std::hash::Hash + std::cmp::Eq,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<HashMap<TKey, TValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<TKey>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<HashMap<TKey, TValue>>,
}

impl<TKey, TValue> From<HashMap<TKey, TValue>> for UpdateMap<TKey, TValue>
where
    TKey: std::hash::Hash + std::cmp::Eq,
{
    fn from(el: HashMap<TKey, TValue>) -> Self {
        UpdateMap {
            set: Some(el),
            remove: None,
            add: None,
        }
    }
}

impl<TKey, TValue> From<Option<HashMap<TKey, TValue>>> for UpdateMap<TKey, TValue>
where
    TKey: std::hash::Hash + std::cmp::Eq,
{
    fn from(el: Option<HashMap<TKey, TValue>>) -> Self {
        match el {
            Some(x) => x.into(),
            None => UpdateMap {
                set: Some(HashMap::new()),
                remove: None,
                add: None,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
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
