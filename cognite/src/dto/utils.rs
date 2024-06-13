use serde::{de::Visitor, Deserialize, Serialize};

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

        impl<'de> Visitor<'de> for MaybeStringVisitor {
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
