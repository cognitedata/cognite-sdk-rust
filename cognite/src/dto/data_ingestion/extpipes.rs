use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Identity, IntoPatch, IntoPatchItem, Patch, Range, UpdateList, UpdateMap, UpdateSet};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
/// Reference to a raw table in an extraction pipeline
pub struct ExtPipeRawTable {
    /// Raw database name
    pub db_name: String,
    /// Raw table name
    pub table_name: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
/// Contact person for an extraction pipeline
pub struct ExtPipeContact {
    /// Name of contact.
    pub name: Option<String>,
    /// Contact e-mail.
    pub email: Option<String>,
    /// Contact role.
    pub role: Option<String>,
    /// Whether to send a notification to this person when
    /// the status of an extraciton pipeline changes.
    pub send_notification: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// An extraction pipeline.
pub struct ExtPipe {
    /// Internal ID
    pub id: i64,
    /// Extraction pipeline external ID. Must be unique accross all extraction pipelines in the project.
    pub external_id: String,
    /// Extraction pipeline name.
    pub name: String,
    /// Short description.
    pub description: Option<String>,
    /// Data set this extraction pipeline belongs to.
    pub data_set_id: i64,
    /// List of raw tables the extractor this extraction pipeline represents is documented
    /// to write to.
    pub raw_tables: Option<Vec<ExtPipeRawTable>>,
    /// Documented schedule.
    pub schedule: Option<String>,
    /// List of contacts.
    pub contacts: Option<Vec<ExtPipeContact>>,
    /// Application specific metadata.
    pub metadata: Option<HashMap<String, String>>,
    /// User provided source.
    pub source: Option<String>,
    /// Long form documentation.
    pub documentation: Option<String>,
    /// Timestamp of last success, in milliseconds since epoch.
    pub last_success: Option<i64>,
    /// Timestamp of last failure, in milliseconds since epoch.
    pub last_failure: Option<i64>,
    /// Last message received.
    pub last_message: Option<String>,
    /// Timestamp of last event, in milliseconds since epoch.
    pub last_seen: Option<i64>,
    /// Time this extraction pipeline was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this extraction pipeline was last updated, in milliseconds since epoch.
    pub last_updated_time: i64,
    /// The user that created this extraction pipeline, if available.
    pub created_by: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Create an extraction pipeline.
pub struct AddExtPipe {
    /// Extraction pipeline external ID. Must be unique accross all extraction pipelines in the project.
    pub external_id: String,
    /// Extraction pipeline name.
    pub name: String,
    /// Short description.
    pub description: Option<String>,
    /// Data set this extraction pipeline belongs to.
    pub data_set_id: i64,
    /// List of raw tables the extractor this extraction pipeline represents is documented
    /// to write to.
    pub raw_tables: Option<Vec<ExtPipeRawTable>>,
    /// Documented schedule.
    pub schedule: Option<String>,
    /// List of contacts.
    pub contacts: Option<Vec<ExtPipeContact>>,
    /// Application specific metadata.
    pub metadata: Option<HashMap<String, String>>,
    /// User provided source.
    pub source: Option<String>,
    /// Long form documentation.
    pub documentation: Option<String>,
}

impl From<ExtPipe> for AddExtPipe {
    fn from(pipe: ExtPipe) -> Self {
        AddExtPipe {
            external_id: pipe.external_id,
            name: pipe.name,
            description: pipe.description,
            data_set_id: pipe.data_set_id,
            raw_tables: pipe.raw_tables,
            schedule: pipe.schedule,
            contacts: pipe.contacts,
            metadata: pipe.metadata,
            source: pipe.source,
            documentation: pipe.documentation,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Update an extraction pipeline
pub struct PatchExtPipe {
    /// Extraction pipeline external ID. Must be unique accross all extraction pipelines in the project.
    pub external_id: Option<UpdateSet<String>>,
    /// Extraction pipeline name.
    pub name: Option<UpdateSet<String>>,
    /// Short description.
    pub description: Option<UpdateSet<Option<String>>>,
    /// Data set this extraction pipeline belongs to.
    pub data_set_id: Option<UpdateSet<i64>>,
    /// Documented schedule.
    pub schedule: Option<UpdateSet<Option<String>>>,
    /// List of raw tables the extractor this extraction pipeline represents is documented
    /// to write to.
    pub raw_tables: Option<UpdateList<ExtPipeRawTable, ExtPipeRawTable>>,
    /// List of contacts.
    pub contacts: Option<UpdateList<ExtPipeContact, ExtPipeContact>>,
    /// Application specific metadata.
    pub metadata: Option<UpdateMap<String, String>>,
    /// User provided source.
    pub source: Option<UpdateSet<Option<String>>>,
    /// Long form documentation.
    pub documentation: Option<UpdateSet<Option<String>>>,
}

impl IntoPatch<Patch<PatchExtPipe>> for ExtPipe {
    fn patch(self, ignore_nulls: bool) -> Patch<PatchExtPipe> {
        Patch::<PatchExtPipe> {
            id: Identity::ExternalId {
                external_id: self.external_id,
            },
            update: PatchExtPipe {
                external_id: None,
                name: self.name.patch(ignore_nulls),
                description: self.description.patch(ignore_nulls),
                data_set_id: self.data_set_id.patch(ignore_nulls),
                schedule: self.schedule.patch(ignore_nulls),
                raw_tables: self.raw_tables.patch(ignore_nulls),
                contacts: self.contacts.patch(ignore_nulls),
                metadata: self.metadata.patch(ignore_nulls),
                source: self.source.patch(ignore_nulls),
                documentation: self.documentation.patch(ignore_nulls),
            },
        }
    }
}

impl IntoPatch<PatchExtPipe> for AddExtPipe {
    fn patch(self, ignore_nulls: bool) -> PatchExtPipe {
        PatchExtPipe {
            external_id: self.external_id.patch(ignore_nulls),
            name: self.name.patch(ignore_nulls),
            description: self.description.patch(ignore_nulls),
            data_set_id: self.data_set_id.patch(ignore_nulls),
            schedule: self.schedule.patch(ignore_nulls),
            raw_tables: self.raw_tables.patch(ignore_nulls),
            contacts: self.contacts.patch(ignore_nulls),
            metadata: self.metadata.patch(ignore_nulls),
            source: self.source.patch(ignore_nulls),
            documentation: self.documentation.patch(ignore_nulls),
        }
    }
}

impl From<ExtPipe> for Patch<PatchExtPipe> {
    fn from(sequence: ExtPipe) -> Self {
        IntoPatch::<Patch<PatchExtPipe>>::patch(sequence, false)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Status of an extraction pipeline run.
pub enum ExtPipeRunStatus {
    /// Success, the run completed succesfully.
    Success,
    /// Failure, the run failed.
    Failure,
    /// Seen, the run is a heartbeat.
    Seen,
}

impl Default for ExtPipeRunStatus {
    fn default() -> Self {
        Self::Seen
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone)]
/// Filter extraction pipelines.
pub struct ExtPipeFilter {
    /// Filter using this (case-sensitive) prefix on external ID.
    pub external_id_prefix: Option<String>,
    /// Include extraction pipelines with this name.
    pub name: Option<String>,
    /// Include extraction pipelines with this description.
    pub description: Option<String>,
    /// Include extraction pipelines which belongs to one of these data set ids.
    pub data_set_ids: Option<Vec<Identity>>,
    /// Include extraction pipelines with this schedule.
    pub schedule: Option<String>,
    /// Include extraction pipelines with these contacts.
    pub contacts: Option<Vec<ExtPipeContact>>,
    /// Include extraction pipelines with these raw tables.
    pub raw_tables: Option<Vec<ExtPipeRawTable>>,
    /// Include extraction pipelines with this metadata.
    pub metadata: Option<HashMap<String, String>>,
    /// Include extraction pipelines with this source.
    pub source: Option<String>,
    /// Include extraction pipelines with this documentation.
    pub documentation: Option<String>,
    /// Include extraction pipelines with this creator.
    pub created_by: Option<String>,
    /// Range of timestamps for `created_time`.
    pub created_time: Option<Range<i64>>,
    /// Range of timestamps for `last_updated_time`.
    pub last_updated_time: Option<Range<i64>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A single completed run or heartbeat.
pub struct ExtPipeRun {
    /// Run ID.
    pub id: i64,
    /// Run status
    pub status: ExtPipeRunStatus,
    /// Optional message.
    pub message: Option<String>,
    /// Time this run happened, in milliseconds since epoch.
    pub created_time: i64,
    /// Extraction pipeline external ID.
    pub external_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Create a new extraction pipeline run.
pub struct AddExtPipeRun {
    /// Run status
    pub status: ExtPipeRunStatus,
    /// Optional message.
    pub message: Option<String>,
    /// Time this run happened, in milliseconds since epoch.
    pub created_time: Option<i64>,
    /// Extraction pipeline external ID.
    pub external_id: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Filter on a string in extraction pipeline runs.
pub struct ExtPipeStringFilter {
    /// Match on a substring of the filtered value.
    pub substring: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Filter extraction pipeline runs.
pub struct ExtPipeRunFilter {
    /// Extraction pipeline external ID.
    pub external_id: String,
    /// Include runs with one of these statuses.
    pub statuses: Option<Vec<ExtPipeRunStatus>>,
    /// Include runs within this range.
    pub created_time: Option<Range<i64>>,
    /// Include runs with messages matching this filter.
    pub message: Option<ExtPipeStringFilter>,
}
