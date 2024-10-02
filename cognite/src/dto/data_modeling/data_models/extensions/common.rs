use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::instances::InstanceId;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CogniteSourceable {
    /// Identifier from the source system.
    pub source_id: Option<String>,
    /// Context of the source id. For systems where the sourceId is globally unique, the sourceContext is expected to not be set.
    pub source_context: Option<String>,
    /// Direct relation to a source system.
    pub source: Option<InstanceId>,
    /// When the instance was created in source system (if available).
    pub source_created_time: Option<i64>,
    /// When the instance was last updated in the source system (if available)
    pub source_updated_time: Option<i64>,
    /// User identifier from the source system on who created the source data. This identifier is
    /// not guaranteed to match the user identifiers in CDF.
    pub source_created_user: Option<String>,
    /// User identifier from the source system on who last updated the source data.
    /// This identifier is not guaranteed to match the user identifiers in CDF.
    pub source_updated_user: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CogniteDescribable {
    /// Name of the instance.
    pub name: String,
    /// Description of the instance.
    pub description: Option<String>,
    /// Text based labels for generic use, limited to 1000.
    pub tags: Option<Vec<String>>,
    /// Alternative names for the node.
    pub aliases: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CogniteAuditable {
    pub last_updated_time: i64,
    pub created_time: i64,
    pub deleted_time: Option<i64>,
}
