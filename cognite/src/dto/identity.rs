use std::collections::HashMap;

use serde::{ser::SerializeSeq, Deserialize, Serialize};

use crate::{
    models::instances::InstanceId, ApiErrorDetail, Chunkable, FromErrorDetail,
    IntegerStringOrObject,
};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(untagged, rename_all_fields = "camelCase")]
/// An Identity represents a CDF resource either by internal ID or external ID.
pub enum Identity {
    /// Identity by CDF internal ID.
    Id {
        /// Numerical internal ID.
        id: i64,
    },
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

impl From<&String> for Identity {
    fn from(value: &String) -> Self {
        Self::from(value.clone())
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

impl<'a> From<&'a str> for CogniteExternalId {
    fn from(external_id: &'a str) -> Self {
        CogniteExternalId {
            external_id: external_id.to_owned(),
        }
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

impl From<&String> for IdentityOrInstance {
    fn from(value: &String) -> Self {
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

/// Serializable wrapper around a list of identities.
/// This implements serialize for individual strings or integers.
/// as well as lists of these. This is useful for the many endpoints that
/// accept lists of identities.
pub struct IdentityList<R>(R);

impl<'a, R> Chunkable<'a> for IdentityList<R>
where
    R: Chunkable<'a>,
{
    type Chunk = IdentityList<R::Chunk>;

    fn as_chunks(&'a self, chunk_size: usize) -> impl Iterator<Item = Self::Chunk> {
        self.0.as_chunks(chunk_size).map(IdentityList)
    }
}

impl<R> From<R> for IdentityList<R> {
    fn from(value: R) -> Self {
        IdentityList(value)
    }
}

/// Serializable wrapper around a list of identity or instance IDs.
/// This implements serialize for individual strings, integers or instance IDs,
/// as well as lists of these.
/// This is useful for the many endpoints that accept lists of identities.
pub struct IdentityOrInstanceList<R>(R);

impl<T> Serialize for IdentityOrInstanceList<T>
where
    IdentityList<T>: Serialize,
    T: Copy,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        IdentityList(self.0).serialize(serializer)
    }
}

impl<'a, R> Chunkable<'a> for IdentityOrInstanceList<R>
where
    R: Chunkable<'a>,
{
    type Chunk = IdentityOrInstanceList<R::Chunk>;

    fn as_chunks(&'a self, chunk_size: usize) -> impl Iterator<Item = Self::Chunk> {
        self.0.as_chunks(chunk_size).map(IdentityOrInstanceList)
    }
}

impl<R> From<R> for IdentityOrInstanceList<R> {
    fn from(value: R) -> Self {
        IdentityOrInstanceList(value)
    }
}

macro_rules! identity_list_ser_directly {
    ($r:ident, $t:ty) => {
        impl Serialize for $r<$t> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.0.serialize(serializer)
            }
        }
    };
    ($r:ident, $t:ty, $n:ident) => {
        impl<const $n: usize> Serialize for $r<$t> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.0.serialize(serializer)
            }
        }
    };
}

identity_list_ser_directly!(IdentityList, &Vec<Identity>);
identity_list_ser_directly!(IdentityOrInstanceList, &Vec<IdentityOrInstance>);
identity_list_ser_directly!(IdentityList, &Vec<CogniteExternalId>);
identity_list_ser_directly!(IdentityList, &Vec<CogniteId>);
identity_list_ser_directly!(IdentityList, &Vec<&Identity>);
identity_list_ser_directly!(IdentityOrInstanceList, &Vec<&IdentityOrInstance>);
identity_list_ser_directly!(IdentityList, &Vec<&CogniteExternalId>);
identity_list_ser_directly!(IdentityList, &Vec<&CogniteId>);
identity_list_ser_directly!(IdentityList, &[Identity]);
identity_list_ser_directly!(IdentityOrInstanceList, &[IdentityOrInstance]);
identity_list_ser_directly!(IdentityList, &[CogniteExternalId]);
identity_list_ser_directly!(IdentityList, &[CogniteId]);
identity_list_ser_directly!(IdentityList, &[&Identity]);
identity_list_ser_directly!(IdentityOrInstanceList, &[&IdentityOrInstance]);
identity_list_ser_directly!(IdentityList, &[&CogniteExternalId]);
identity_list_ser_directly!(IdentityList, &[&CogniteId]);
identity_list_ser_directly!(IdentityList, &[Identity; N], N);
identity_list_ser_directly!(IdentityOrInstanceList, &[IdentityOrInstance; N], N);
identity_list_ser_directly!(IdentityList, &[CogniteExternalId; N], N);
identity_list_ser_directly!(IdentityList, &[CogniteId; N], N);
identity_list_ser_directly!(IdentityList, &[&Identity; N], N);
identity_list_ser_directly!(IdentityOrInstanceList, &[&IdentityOrInstance; N], N);
identity_list_ser_directly!(IdentityList, &[&CogniteExternalId; N], N);
identity_list_ser_directly!(IdentityList, &[&CogniteId; N], N);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExternalIdRef<'a, T> {
    external_id: &'a T,
}

macro_rules! identity_list_ser_external_id {
    ($t:ty) => {
        impl Serialize for IdentityList<$t> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
                for id in self.0.iter() {
                    seq.serialize_element(&ExternalIdRef { external_id: id })?;
                }
                seq.end()
            }
        }
    };

    ($t:ty, $n:ident) => {
        impl<const $n: usize> Serialize for IdentityList<$t> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(Some($n))?;
                for id in self.0.iter() {
                    seq.serialize_element(&ExternalIdRef { external_id: id })?;
                }
                seq.end()
            }
        }
    };
}
identity_list_ser_external_id!(&Vec<String>);
identity_list_ser_external_id!(&[String]);
identity_list_ser_external_id!(&[String; N], N);
identity_list_ser_external_id!(&Vec<&String>);
identity_list_ser_external_id!(&[&String]);
identity_list_ser_external_id!(&[&String; N], N);
identity_list_ser_external_id!(&Vec<&str>);
identity_list_ser_external_id!(&[&str]);
identity_list_ser_external_id!(&[&str; N], N);

macro_rules! identity_list_ser_id {
    ($t:ty) => {
        impl Serialize for IdentityList<$t> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
                for id in self.0.iter() {
                    seq.serialize_element(&CogniteId { id: *id })?;
                }
                seq.end()
            }
        }
    };

    ($t:ty, $n:ident) => {
        impl<const N: usize> Serialize for IdentityList<$t> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(N))?;
                for id in self.0.iter() {
                    seq.serialize_element(&CogniteId { id: *id })?;
                }
                seq.end()
            }
        }
    };
}

identity_list_ser_id!(&Vec<i64>);
identity_list_ser_id!(&[i64]);
identity_list_ser_id!(&[i64; N], N);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InstanceIdRef<'a> {
    instance_id: &'a InstanceId,
}

macro_rules! identity_list_ser_instance_id {
    ($t:ty) => {
        impl Serialize for IdentityOrInstanceList<$t> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
                for id in self.0.iter() {
                    seq.serialize_element(&InstanceIdRef { instance_id: id })?;
                }
                seq.end()
            }
        }
    };

    ($t:ty, $n:ident) => {
        impl<const N: usize> Serialize for IdentityOrInstanceList<$t> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(N))?;
                for id in self.0.iter() {
                    seq.serialize_element(&InstanceIdRef { instance_id: id })?;
                }
                seq.end()
            }
        }
    };
}

identity_list_ser_instance_id!(&Vec<InstanceId>);
identity_list_ser_instance_id!(&[InstanceId]);
identity_list_ser_instance_id!(&[InstanceId; N], N);
identity_list_ser_instance_id!(&Vec<&InstanceId>);
identity_list_ser_instance_id!(&[&InstanceId]);
identity_list_ser_instance_id!(&[&InstanceId; N], N);

macro_rules! identity_list_ser_single {
    ($r:ident, $t:ty) => {
        impl Serialize for $r<$t> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                Serialize::serialize(&$r(&[self.0]), serializer)
            }
        }
    };
}

macro_rules! impl_chunk_single {
    ($t:ty) => {
        impl<'a> Chunkable<'a> for $t {
            type Chunk = &'a $t;
            fn as_chunks(&'a self, _chunk_size: usize) -> impl Iterator<Item = Self::Chunk> {
                std::iter::once(self)
            }
        }
    };
}

identity_list_ser_single!(IdentityList, i64);

impl Chunkable<'_> for i64 {
    type Chunk = i64;
    fn as_chunks(&self, _chunk_size: usize) -> impl Iterator<Item = Self::Chunk> {
        std::iter::once(*self)
    }
}

identity_list_ser_single!(IdentityList, &str);
impl_chunk_single!(&'a str);
identity_list_ser_single!(IdentityList, &String);
impl_chunk_single!(&'a String);
identity_list_ser_single!(IdentityOrInstanceList, &InstanceId);
impl_chunk_single!(&'a InstanceId);
identity_list_ser_single!(IdentityList, &Identity);
impl_chunk_single!(&'a Identity);
identity_list_ser_single!(IdentityOrInstanceList, &IdentityOrInstance);
impl_chunk_single!(&'a IdentityOrInstance);
identity_list_ser_single!(IdentityList, &CogniteExternalId);
impl_chunk_single!(&'a CogniteExternalId);
identity_list_ser_single!(IdentityList, &CogniteId);
impl_chunk_single!(&'a CogniteId);

#[cfg(test)]
mod tests {
    use crate::{
        models::instances::InstanceId, Chunkable, CogniteExternalId, CogniteId, IdentityOrInstance,
        IdentityOrInstanceList,
    };

    use super::{Identity, IdentityList};

    macro_rules! test_identity_list {
        ($v:expr) => {
            let v = $v;
            let list = IdentityList::from(&v);
            let serialized = serde_json::to_string(&list).unwrap();
            let deserialized: Vec<Identity> = serde_json::from_str(&serialized).unwrap();
            let reserialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, reserialized);

            let chunks = list.as_chunks(2).collect::<Vec<_>>();
            assert_eq!(chunks.len(), 2);
            for chunk in chunks {
                assert_eq!(chunk.0.len(), 2);
            }
        };
    }

    macro_rules! test_identity_or_instance_list {
        ($v:expr) => {
            let v = $v;
            let list = IdentityOrInstanceList::from(&v);
            let serialized = serde_json::to_string(&list).unwrap();
            let deserialized: Vec<IdentityOrInstance> = serde_json::from_str(&serialized).unwrap();
            let reserialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(serialized, reserialized);

            let chunks = list.as_chunks(2).collect::<Vec<_>>();
            assert_eq!(chunks.len(), 2);
            for chunk in chunks {
                assert_eq!(chunk.0.len(), 2);
            }
        };
    }

    #[test]
    fn test_identity_list() {
        test_identity_list!(vec![
            Identity::external_id("ext1"),
            Identity::id(2),
            Identity::external_id("ext3"),
            Identity::id(4)
        ]);
        test_identity_list!(vec![
            CogniteExternalId::from("ext1"),
            CogniteExternalId::from("ext2"),
            CogniteExternalId::from("ext3"),
            CogniteExternalId::from("ext4")
        ]);
        test_identity_list!(vec![
            CogniteId::from(1),
            CogniteId::from(2),
            CogniteId::from(3),
            CogniteId::from(4)
        ]);
        test_identity_list!([
            Identity::external_id("ext1"),
            Identity::id(2),
            Identity::external_id("ext3"),
            Identity::id(4)
        ]);

        test_identity_list!(vec!["ext1", "ext2", "ext3", "ext4"]);
        test_identity_list!(vec![1i64, 2, 3, 4]);
        test_identity_list!(["ext1", "ext2", "ext3", "ext4"]);
        test_identity_list!([1i64, 2, 3, 4]);
    }

    #[test]
    fn test_identity_or_instance_list() {
        use crate::models::instances::InstanceId;
        test_identity_or_instance_list!(vec![
            IdentityOrInstance::external_id("ext1"),
            IdentityOrInstance::id(2),
            IdentityOrInstance::instance_id(InstanceId {
                space: "space1".to_owned(),
                external_id: "inst1".to_owned()
            }),
            IdentityOrInstance::id(4)
        ]);
        test_identity_or_instance_list!([
            IdentityOrInstance::external_id("ext1"),
            IdentityOrInstance::id(2),
            IdentityOrInstance::instance_id(InstanceId {
                space: "space1".to_owned(),
                external_id: "inst1".to_owned()
            }),
            IdentityOrInstance::id(4)
        ]);
        test_identity_or_instance_list!(vec![
            InstanceId {
                space: "space1".to_owned(),
                external_id: "inst1".to_owned()
            },
            InstanceId {
                space: "space2".to_owned(),
                external_id: "inst2".to_owned()
            },
            InstanceId {
                space: "space3".to_owned(),
                external_id: "inst3".to_owned()
            },
            InstanceId {
                space: "space4".to_owned(),
                external_id: "inst4".to_owned()
            }
        ]);
        test_identity_or_instance_list!(["ext1", "ext2", "ext3", "ext4"]);
        test_identity_or_instance_list!([1i64, 2, 3, 4]);
    }

    #[test]
    fn test_identity_list_single() {
        let x = "extId";
        let list = IdentityList::from(x);
        let serialized = serde_json::to_string(&list).unwrap();
        assert_eq!(serialized, r#"[{"externalId":"extId"}]"#);

        let x = 42i64;
        let list = IdentityList::from(x);
        let serialized = serde_json::to_string(&list).unwrap();
        assert_eq!(serialized, r#"[{"id":42}]"#);

        let x = InstanceId {
            space: "space1".to_owned(),
            external_id: "inst1".to_owned(),
        };
        let list = IdentityOrInstanceList::from(&x);
        let serialized = serde_json::to_string(&list).unwrap();
        assert_eq!(
            serialized,
            r#"[{"instanceId":{"space":"space1","externalId":"inst1"}}]"#
        );
    }
}
