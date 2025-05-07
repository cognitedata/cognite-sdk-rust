use serde::{Deserialize, Serialize};

use crate::IntoParams;

/// A stream to create in CDF.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamWrite {
    /// Stream identifier. The identifier must be unique within the project
    /// and must be a valid stream identifier. Stream identifiers can
    /// only consist of alphanumeric characters, hyphens, and underscores.
    /// It must not start with cdf_ or cognite_, as those are reserved
    /// for future use. Stream id cannot be logs or records.
    /// Max length is 100 characters.
    pub external_id: String,
}

impl From<Stream> for StreamWrite {
    fn from(stream: Stream) -> Self {
        Self {
            external_id: stream.external_id,
        }
    }
}

/// A stream in CDF.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    /// Stream identifier.
    pub external_id: String,
    /// Time the stream was created.
    pub created_time: i64,
}

/// Query parameters for listing streams.
#[derive(Debug, Default)]
pub struct ListStreamParams {}

impl IntoParams for ListStreamParams {
    fn into_params(self) -> Vec<(String, String)> {
        vec![]
    }
}
