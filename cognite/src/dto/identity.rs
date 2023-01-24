use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Identity {
    Id {
        id: i64,
    },
    #[serde(rename_all = "camelCase")]
    ExternalId {
        external_id: String,
    },
}

impl Default for Identity {
    fn default() -> Self {
        Identity::Id { id: 0 }
    }
}

impl From<i64> for Identity {
    fn from(id: i64) -> Self {
        Identity::Id { id }
    }
}

impl From<String> for Identity {
    fn from(external_id: String) -> Self {
        Identity::ExternalId { external_id }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CogniteId {
    id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CogniteExternalId {
    pub external_id: String,
}

pub trait EqIdentity {
    fn eq(&self, id: &Identity) -> bool;
}

impl From<String> for CogniteExternalId {
    fn from(external_id: String) -> Self {
        CogniteExternalId { external_id }
    }
}

impl From<i64> for CogniteId {
    fn from(id: i64) -> Self {
        CogniteId { id }
    }
}
