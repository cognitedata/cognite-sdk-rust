use crate::{Error, Identity, Result};

/// Given a result object from CDF, if it is a "conflict" error,
/// return the list of identities.
///
/// # Arguments
///
/// * `res` - Result from a CDF request.
pub fn get_duplicates_from_result<T>(res: &Result<T>) -> Option<Vec<Identity>> {
    match res {
        Ok(_) => None,
        Err(e) => match &e {
            Error::Conflict(c) => c
                .duplicated
                .as_ref()
                .map(|dup| dup.get_identities().collect()),
            _ => None,
        },
    }
}

/// Given a result object from CDF, if it is a "missing" error,
/// return the list of identities.
///
/// # Arguments
///
/// * `res` - Result from a CDF request.
pub fn get_missing_from_result<T>(res: &Result<T>) -> Option<Vec<Identity>> {
    match res {
        Ok(_) => None,
        Err(e) => match &e {
            Error::BadRequest(c) => c.missing.as_ref().map(|mis| mis.get_identities().collect()),
            Error::NotFound(c) => c.missing.as_ref().map(|mis| mis.get_identities().collect()),
            _ => None,
        },
    }
}
