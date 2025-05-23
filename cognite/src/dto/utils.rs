use std::collections::HashMap;

use serde::{de::Visitor, Deserialize, Serialize};

use crate::{
    models::{instances::PropertiesObject, views::ViewReference},
    IntegerStringOrObject,
};

/// Wrapper around an u64 value that can be deserialized from
/// a string.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(transparent)]
pub struct MaybeStringU64(pub u64);

impl MaybeStringU64 {
    /// Create a new MaybeString around a given value.
    pub fn new(v: u64) -> Self {
        Self(v)
    }
}

impl<'de> Deserialize<'de> for MaybeStringU64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MaybeStringVisitor;

        impl Visitor<'_> for MaybeStringVisitor {
            type Value = MaybeStringU64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or integer")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(MaybeStringU64::new(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(MaybeStringU64::new(
                    v.parse()
                        .map_err(|_| E::custom("failed to parse integer"))?,
                ))
            }
        }

        deserializer.deserialize_any(MaybeStringVisitor)
    }
}

/// Trait implemented for types that can be retrieved from an error detail element.
pub trait FromErrorDetail: Sized {
    /// Try to obtain a new instance of self from the detail object.
    fn from_detail(detail: &HashMap<String, Box<IntegerStringOrObject>>) -> Option<Self>;
}

/// Get instance type of special data models type.
///
/// # Arguments
///
/// * `view` - View reference of source.
/// # `properties` - Instance properties object of special type.
pub fn get_instance_properties<'a, TProperties>(
    view: &ViewReference,
    properties: &'a mut PropertiesObject<TProperties>,
) -> Option<&'a TProperties> {
    let space = view.space.to_owned();
    let key = format!("{}/{}", view.external_id, view.version);

    properties.get_mut(&space).and_then(|v| v.get(&key))
}
