use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    models::instances::InstanceId, ApiErrorDetail, FromErrorDetail, IntegerStringOrObject,
};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
/// An Identity represents a CDF resource either by internal ID or external ID.
pub enum Identity {
    /// Identity by CDF internal ID.
    Id {
        /// Numerical internal ID.
        id: i64,
    },
    #[serde(rename_all = "camelCase")]
    /// Identity by CDF external ID.
    ExternalId {
        /// External ID, unique for the given resource.
        external_id: String,
    },
}

impl Identity {
    /// Create an identity using a CDF internal ID.
    pub fn id(id: i64) -> Self {
        Identity::Id { id }
    }

    /// Create an identity using a CDF external ID.
    pub fn external_id(external_id: impl Into<String>) -> Self {
        Identity::ExternalId {
            external_id: external_id.into(),
        }
    }

    /// Consume self and return Some if this is an external ID.
    pub fn into_external_id(self) -> Option<String> {
        match self {
            Identity::ExternalId { external_id } => Some(external_id),
            _ => None,
        }
    }

    /// Consume self and return Some if this is an internal ID.
    pub fn into_id(self) -> Option<i64> {
        match self {
            Identity::Id { id } => Some(id),
            _ => None,
        }
    }

    /// Get self as external ID.
    pub fn as_external_id(&self) -> Option<&String> {
        match self {
            Identity::ExternalId { external_id } => Some(external_id),
            _ => None,
        }
    }

    /// Get self as internal ID.
    pub fn as_id(&self) -> Option<i64> {
        match self {
            Identity::Id { id } => Some(*id),
            _ => None,
        }
    }
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

impl From<&str> for Identity {
    fn from(external_id: &str) -> Self {
        Identity::ExternalId {
            external_id: external_id.to_owned(),
        }
    }
}

impl FromErrorDetail for Identity {
    fn from_detail(detail: &HashMap<String, Box<IntegerStringOrObject>>) -> Option<Self> {
        ApiErrorDetail::get_integer(detail, "id")
            .map(|id| Identity::Id { id })
            .or_else(|| {
                ApiErrorDetail::get_string(detail, "externalId").map(|id| Identity::ExternalId {
                    external_id: id.to_owned(),
                })
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Wrapper around a cognite internal ID.
pub struct CogniteId {
    /// Internal ID.
    id: i64,
}

impl FromErrorDetail for CogniteId {
    fn from_detail(detail: &HashMap<String, Box<IntegerStringOrObject>>) -> Option<Self> {
        ApiErrorDetail::get_integer(detail, "id").map(|id| CogniteId { id })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// Wrapper around a cognite external ID.
pub struct CogniteExternalId {
    /// External ID.
    pub external_id: String,
}

impl FromErrorDetail for CogniteExternalId {
    fn from_detail(detail: &HashMap<String, Box<IntegerStringOrObject>>) -> Option<Self> {
        ApiErrorDetail::get_string(detail, "externalId").map(|external_id| CogniteExternalId {
            external_id: external_id.to_owned(),
        })
    }
}

/// Trait indicating that a type can be compared to an identity.
pub trait EqIdentity {
    /// Return true if the identity given by `id` points to self.
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

#[derive(Debug, Deserialize, Serialize, Clone, Hash, Eq, PartialEq)]
#[serde(rename_all_fields = "camelCase")]
#[serde(untagged)]
/// Identity or instance ID.
pub enum IdentityOrInstance {
    /// Identity, external ID or internal ID.
    Identity(Identity),
    /// Instance ID, refering to a node in data modeling.
    InstanceId {
        /// Instance id.
        instance_id: InstanceId,
    },
}

impl IdentityOrInstance {
    /// Create a new IdentityOrInstance using an internal ID.
    pub fn id(id: i64) -> Self {
        Self::Identity(Identity::id(id))
    }

    /// Create a new IdentityOrInstance using an external ID.
    pub fn external_id(external_id: impl Into<String>) -> Self {
        Self::Identity(Identity::external_id(external_id))
    }

    /// Create a new IdentityOrInstance using an instance ID.
    pub fn instance_id(instance_id: impl Into<InstanceId>) -> Self {
        Self::InstanceId {
            instance_id: instance_id.into(),
        }
    }

    /// Get self as internal ID.
    pub fn as_id(&self) -> Option<i64> {
        match self {
            Self::Identity(i) => i.as_id(),
            _ => None,
        }
    }

    /// Get self as external ID.
    pub fn as_external_id(&self) -> Option<&String> {
        match self {
            Self::Identity(i) => i.as_external_id(),
            _ => None,
        }
    }

    /// Get self as identity.
    pub fn as_identity(&self) -> Option<&Identity> {
        match self {
            Self::Identity(i) => Some(i),
            _ => None,
        }
    }

    /// Get self as instance id.
    pub fn as_instance_id(&self) -> Option<&InstanceId> {
        match self {
            Self::InstanceId { instance_id: i } => Some(i),
            _ => None,
        }
    }
}

// Default impl is not super meaningful, but is useful in some cases
impl Default for IdentityOrInstance {
    fn default() -> Self {
        Self::Identity(Default::default())
    }
}

impl From<&str> for IdentityOrInstance {
    fn from(value: &str) -> Self {
        Self::Identity(Identity::from(value))
    }
}

impl From<String> for IdentityOrInstance {
    fn from(value: String) -> Self {
        Self::Identity(Identity::from(value))
    }
}

impl From<i64> for IdentityOrInstance {
    fn from(value: i64) -> Self {
        Self::Identity(Identity::from(value))
    }
}

impl From<Identity> for IdentityOrInstance {
    fn from(value: Identity) -> Self {
        Self::Identity(value)
    }
}

impl From<InstanceId> for IdentityOrInstance {
    fn from(value: InstanceId) -> Self {
        Self::InstanceId { instance_id: value }
    }
}

impl PartialEq<i64> for IdentityOrInstance {
    fn eq(&self, other: &i64) -> bool {
        self.as_id().as_ref() == Some(other)
    }
}

impl PartialEq<str> for IdentityOrInstance {
    fn eq(&self, other: &str) -> bool {
        self.as_external_id().map(|a| a.as_str()) == Some(other)
    }
}

impl PartialEq<InstanceId> for IdentityOrInstance {
    fn eq(&self, other: &InstanceId) -> bool {
        self.as_instance_id() == Some(other)
    }
}

impl PartialEq<Identity> for IdentityOrInstance {
    fn eq(&self, other: &Identity) -> bool {
        self.as_identity() == Some(other)
    }
}

impl FromErrorDetail for IdentityOrInstance {
    fn from_detail(detail: &HashMap<String, Box<IntegerStringOrObject>>) -> Option<Self> {
        if let Some(object) = ApiErrorDetail::get_object(detail, "instanceId") {
            match (
                ApiErrorDetail::get_string(object, "space"),
                ApiErrorDetail::get_string(object, "externalId"),
            ) {
                (Some(space), Some(external_id)) => Some(Self::InstanceId {
                    instance_id: InstanceId {
                        space: space.to_owned(),
                        external_id: external_id.to_owned(),
                    },
                }),
                _ => None,
            }
        } else {
            Identity::from_detail(detail).map(Self::Identity)
        }
    }
}
