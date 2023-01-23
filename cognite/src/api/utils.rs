use crate::{Identity, Kind, Result};

pub fn get_duplicates_from_result<T>(res: &Result<T>) -> Option<Vec<Identity>> {
    match res {
        Ok(_) => None,
        Err(e) => match &e.kind {
            Kind::Conflict(c) => c
                .duplicated
                .as_ref()
                .map(|dup| dup.get_identities().collect()),
            _ => None,
        },
    }
}

pub fn get_missing_from_result<T>(res: &Result<T>) -> Option<Vec<Identity>> {
    match res {
        Ok(_) => None,
        Err(e) => match &e.kind {
            Kind::BadRequest(c) => c.missing.as_ref().map(|mis| mis.get_identities().collect()),
            Kind::NotFound(c) => c.missing.as_ref().map(|mis| mis.get_identities().collect()),
            _ => None,
        },
    }
}
