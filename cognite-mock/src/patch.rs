use cognite::{CogniteExternalId, UpdateList, UpdateMap, UpdateSet, UpdateSetNull};
use std::collections::HashMap;

pub fn apply_set_null<T>(current: Option<T>, patch: Option<UpdateSetNull<T>>) -> Option<T> {
    match patch {
        None => current,
        Some(UpdateSetNull::Set { set }) => Some(set),
        Some(UpdateSetNull::SetNull { set_null: true }) => None,
        Some(UpdateSetNull::SetNull { set_null: false }) => current,
    }
}

pub fn apply_set<T>(current: T, patch: Option<UpdateSet<T>>) -> T {
    match patch {
        None => current,
        Some(UpdateSet { set }) => set,
    }
}

pub fn apply_set_opt<T>(current: Option<T>, patch: Option<UpdateSet<T>>) -> Option<T> {
    match patch {
        None => current,
        Some(UpdateSet { set }) => Some(set),
    }
}

pub fn apply_map<K: std::hash::Hash + Eq, V>(
    current: Option<HashMap<K, V>>,
    patch: Option<UpdateMap<K, V>>,
) -> Option<HashMap<K, V>> {
    match patch {
        None => current,
        Some(UpdateMap::Set { set }) => {
            if set.is_empty() {
                None
            } else {
                Some(set)
            }
        }
        Some(UpdateMap::AddRemove { add, remove }) => {
            let mut map = current.unwrap_or_default();
            if let Some(keys) = remove {
                for k in keys {
                    map.remove(&k);
                }
            }
            if let Some(entries) = add {
                map.extend(entries);
            }
            if map.is_empty() {
                None
            } else {
                Some(map)
            }
        }
    }
}

pub fn apply_list_ext_id(
    current: Option<Vec<CogniteExternalId>>,
    patch: Option<UpdateList<CogniteExternalId, CogniteExternalId>>,
) -> Option<Vec<CogniteExternalId>> {
    match patch {
        None => current,
        Some(UpdateList::Set { set }) => {
            if set.is_empty() {
                None
            } else {
                Some(set)
            }
        }
        Some(UpdateList::AddRemove { add, remove }) => {
            let mut list = current.unwrap_or_default();
            if let Some(remove) = remove {
                list.retain(|x| !remove.iter().any(|r| r.external_id == x.external_id));
            }
            if let Some(add) = add {
                list.extend(add);
            }
            if list.is_empty() {
                None
            } else {
                Some(list)
            }
        }
    }
}

pub fn apply_list_i64(
    current: Option<Vec<i64>>,
    patch: Option<UpdateList<i64, i64>>,
) -> Option<Vec<i64>> {
    match patch {
        None => current,
        Some(UpdateList::Set { set }) => {
            if set.is_empty() {
                None
            } else {
                Some(set)
            }
        }
        Some(UpdateList::AddRemove { add, remove }) => {
            let mut list = current.unwrap_or_default();
            if let Some(remove) = remove {
                list.retain(|x| !remove.contains(x));
            }
            if let Some(add) = add {
                list.extend(add);
            }
            if list.is_empty() {
                None
            } else {
                Some(list)
            }
        }
    }
}

/// Paginate a pre-filtered list using a cursor that encodes the next offset.
pub fn paginate<T>(items: Vec<T>, cursor: Option<&str>, limit: usize) -> (Vec<T>, Option<String>) {
    let offset = cursor.and_then(|c| c.parse::<usize>().ok()).unwrap_or(0);
    let total = items.len();
    let end = (offset + limit).min(total);
    let next_cursor = if end < total {
        Some(end.to_string())
    } else {
        None
    };
    let page = items.into_iter().skip(offset).take(limit).collect();
    (page, next_cursor)
}
