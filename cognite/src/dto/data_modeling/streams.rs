use serde::{Deserialize, Serialize};

use crate::{to_query, IntoParams};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Reference to a template used to create a stream.
pub struct StreamTemplate {
    /// Name of the template.
    pub name: String,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Settings for creating a stream.
pub struct StreamSettingsWrite {
    /// The template to use when creating the stream.
    pub template: StreamTemplate,
}

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
    /// Settings for the stream.
    pub settings: StreamSettingsWrite,
}

impl From<Stream> for StreamWrite {
    fn from(stream: Stream) -> Self {
        Self {
            external_id: stream.external_id,
            settings: StreamSettingsWrite {
                template: StreamTemplate {
                    name: stream.created_from_template,
                },
            },
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// Type of stream.
pub enum StreamType {
    #[default]
    /// The stream is immutable. Immutable streams allow ingestion
    /// of very large amounts of data.
    Immutable,
    /// The stream is mutable. Mutable streams allow modification
    /// and deletion of records, but have lower ingestion limits.
    Mutable,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Lifecycle settings for a stream.
pub struct StreamLifecycle {
    /// ISO-8601 formatted date string for when data in the stream
    /// will be automatically deleted.
    pub data_deleted_after: Option<String>,
    /// ISO-8601 formatted date string for when the stream
    /// will be deleted once it is soft-deleted.
    pub retained_after_soft_delete: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Resource limits and metrics for a stream.
pub struct StreamResourceLimits {
    /// Provisioned capacity for the stream.
    pub provisioned: f64,
    /// Consumed capacity for the stream. Only included if `includeStatistics` is `true`.
    pub consumed: Option<f64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Limits for a stream.
pub struct StreamLimits {
    /// Maximum length of time that the `lastUpdatedTime` filter can retrieve
    /// records for, in ISO-8601 format. This setting is only available for immutable
    /// streams.
    pub max_filtering_interval: Option<String>,
    /// Maximum number of records allowed in the stream.
    pub max_records_total: StreamResourceLimits,
    /// Maximum total size of all records in the stream, in gigabytes.
    pub max_giga_bytes_total: StreamResourceLimits,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Settings for a stream.
pub struct StreamSettings {
    /// Lifecycle settings for the stream.
    pub lifecycle: StreamLifecycle,
    /// Limits for the stream.
    pub limits: StreamLimits,
}

/// A stream in CDF.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    /// Stream identifier.
    pub external_id: String,
    /// Time the stream was created.
    pub created_time: i64,
    /// Name of the template used for creating this stream. Note:
    /// This value is for information only. The template might have been modified
    /// or even entirely deleted after the stream was created.
    pub created_from_template: String,
    /// Defines type of the stream.
    pub r#type: StreamType,
    /// Settings for the stream.
    pub settings: Option<StreamSettings>,
}

/// Query parameters for listing streams.
#[derive(Debug, Default)]
pub struct ListStreamParams {}

impl IntoParams for ListStreamParams {
    fn into_params(self) -> Vec<(String, String)> {
        vec![]
    }
}

#[derive(Debug, Default)]
/// Query parameters for retrieving a stream.
pub struct RetrieveStreamParams {
    /// Whether to include statistics about resource consumption.
    pub include_statistics: Option<bool>,
}

impl IntoParams for RetrieveStreamParams {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        to_query("includeStatistics", &self.include_statistics, &mut params);
        params
    }
}
