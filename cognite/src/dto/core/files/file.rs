use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::{
    models::instances::InstanceId, CogniteExternalId, EqIdentity, Identity, IntoPatch,
    IntoPatchItem, Patch, UpdateList, UpdateMap, UpdateSetNull, UpsertOptions,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Description of a CDF file.
pub struct FileMetadata {
    /// File external ID. Must be unique accross all files in the project.
    pub external_id: Option<String>,
    /// File name.
    pub name: String,
    /// Directory containing the file. Must be an absolute, unix-style path.
    pub directory: Option<String>,
    /// Source of the file.
    pub source: Option<String>,
    /// File mime type, e.g. `application/pdf`.
    pub mime_type: Option<String>,
    /// Custom, application specific metadata. String key -> String value.
    /// Limits: Maximum length of key is 128 bytes, value 10240 bytes,
    /// up to 256 key-value pairs, of total size at most 10240.
    pub metadata: Option<HashMap<String, String>>,
    /// List of assets the file is tied to.
    pub asset_ids: Option<Vec<i64>>,
    /// Data set the file belongs to.
    pub data_set_id: Option<i64>,
    /// Timestamp in milliseconds since epoch when this file was created in the source system.
    pub source_created_time: Option<i64>,
    /// Timestamp in milliseconds since epoch when this file was last modified in the source system.
    pub source_modified_time: Option<i64>,
    /// The required security categories to access this file.
    pub security_categories: Option<Vec<i64>>,
    /// List of labels associated with this file.
    pub labels: Option<Vec<CogniteExternalId>>,
    /// File internal ID.
    pub id: i64,
    /// Whether or not the actual file is uploaded.
    pub uploaded: bool,
    /// Time this file was uploaded, in milliseconds since epoch.
    pub uploaded_time: Option<i64>,
    /// Time this file was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this file was last modified, in milliseconds since epoch.
    pub last_updated_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Extra data in file upload result from normal file upload.
pub struct UploadUrl {
    /// URL for uploading data to this file. Returned only in response to
    /// `upload`.
    pub upload_url: String,
    /// Optional instance id of the file if in data models.
    pub instance_id: Option<InstanceId>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Extra data in file upload result from multipart file upload.
pub struct MultiUploadUrls {
    /// Identifier for this multipart upload, to be used in `complete_multipart_upload`.
    pub upload_id: String,
    /// Upload URL for each part of the file.
    pub upload_urls: Vec<String>,
    /// Optional instance id of the file if in data models.
    pub instance_id: Option<InstanceId>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Result for uploading a file object to CDF.
pub struct FileUploadResult<T> {
    #[serde(flatten)]
    /// File metadata object.
    pub metadata: FileMetadata,
    #[serde(flatten)]
    /// Any extra fields, specific fields depend on endpoint.
    pub extra: T,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Create a new file.
pub struct AddFile {
    /// File external ID. Must be unique accross all files in the project.
    pub external_id: Option<String>,
    /// File name.
    pub name: String,
    /// Directory containing the file. Must be an absolute, unix-style path.
    pub directory: Option<String>,
    /// Source of the file.
    pub source: Option<String>,
    /// File mime type, e.g. `application/pdf`.
    pub mime_type: Option<String>,
    /// Custom, application specific metadata. String key -> String value.
    /// Limits: Maximum length of key is 128 bytes, value 10240 bytes,
    /// up to 256 key-value pairs, of total size at most 10240.
    pub metadata: Option<HashMap<String, String>>,
    /// List of assets the file is tied to.
    pub asset_ids: Option<Vec<i64>>,
    /// Data set the file belongs to.
    pub data_set_id: Option<i64>,
    /// Timestamp in milliseconds since epoch when this file was created in the source system.
    pub source_created_time: Option<i64>,
    /// Timestamp in milliseconds since epoch when this file was last modified in the source system.
    pub source_modified_time: Option<i64>,
    /// The required security categories to access this file.
    pub security_categories: Option<Vec<i64>>,
    /// List of labels associated with this file.
    pub labels: Option<Vec<CogniteExternalId>>,
}

impl From<FileMetadata> for AddFile {
    fn from(file: FileMetadata) -> AddFile {
        AddFile {
            external_id: file.external_id,
            name: file.name,
            directory: file.directory,
            source: file.source,
            mime_type: file.mime_type,
            metadata: file.metadata,
            asset_ids: file.asset_ids,
            data_set_id: file.data_set_id,
            source_created_time: file.source_created_time,
            source_modified_time: file.source_modified_time,
            security_categories: file.security_categories,
            labels: file.labels,
        }
    }
}

impl EqIdentity for AddFile {
    fn eq(&self, id: &Identity) -> bool {
        match id {
            Identity::Id { id: _ } => false,
            Identity::ExternalId { external_id } => self.external_id.as_ref() == Some(external_id),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Update a file.
pub struct PatchFile {
    /// File external ID. Must be unique accross all files in the project.
    pub external_id: Option<UpdateSetNull<String>>,
    /// Directory containing the file. Must be an absolute, unix-style path.
    pub directory: Option<UpdateSetNull<String>>,
    /// Source of the file.
    pub source: Option<UpdateSetNull<String>>,
    /// File mime type, e.g. `application/pdf`.
    pub mime_type: Option<UpdateSetNull<String>>,
    /// Custom, application specific metadata. String key -> String value.
    /// Limits: Maximum length of key is 128 bytes, value 10240 bytes,
    /// up to 256 key-value pairs, of total size at most 10240.
    pub metadata: Option<UpdateMap<String, String>>,
    /// List of assets the file is tied to.
    pub asset_ids: Option<UpdateList<i64, i64>>,
    /// Timestamp in milliseconds since epoch when this file was created in the source system.
    pub source_created_time: Option<UpdateSetNull<i64>>,
    /// Timestamp in milliseconds since epoch when this file was last modified in the source system.
    pub source_modified_time: Option<UpdateSetNull<i64>>,
    /// Data set the file belongs to.
    pub data_set_id: Option<UpdateSetNull<i64>>,
    /// The required security categories to access this file.
    pub security_categories: Option<UpdateList<i64, i64>>,
    /// List of labels associated with this file.
    pub labels: Option<UpdateList<CogniteExternalId, CogniteExternalId>>,
}

impl IntoPatch<Patch<PatchFile>> for FileMetadata {
    fn patch(self, options: &UpsertOptions) -> Patch<PatchFile> {
        Patch::<PatchFile> {
            id: to_idt!(self),
            update: PatchFile {
                external_id: self.external_id.patch(options),
                directory: self.directory.patch(options),
                source: self.source.patch(options),
                mime_type: self.mime_type.patch(options),
                metadata: self.metadata.patch(options),
                asset_ids: self.asset_ids.patch(options),
                source_created_time: self.source_created_time.patch(options),
                source_modified_time: self.source_modified_time.patch(options),
                data_set_id: self.data_set_id.patch(options),
                security_categories: self.security_categories.patch(options),
                labels: self.labels.patch(options),
            },
        }
    }
}

impl IntoPatch<PatchFile> for AddFile {
    fn patch(self, options: &UpsertOptions) -> PatchFile {
        PatchFile {
            external_id: self.external_id.patch(options),
            directory: self.directory.patch(options),
            source: self.source.patch(options),
            mime_type: self.mime_type.patch(options),
            metadata: self.metadata.patch(options),
            asset_ids: self.asset_ids.patch(options),
            source_created_time: self.source_created_time.patch(options),
            source_modified_time: self.source_modified_time.patch(options),
            data_set_id: self.data_set_id.patch(options),
            security_categories: self.security_categories.patch(options),
            labels: self.labels.patch(options),
        }
    }
}

impl From<FileMetadata> for Patch<PatchFile> {
    fn from(file: FileMetadata) -> Self {
        IntoPatch::<Patch<PatchFile>>::patch(file, &Default::default())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Download URL for a file
pub struct FileDownloadUrl {
    #[serde(flatten)]
    /// ID of the file.
    pub id: Identity,
    /// Temporary download URL for the file.
    pub download_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Aggregates on files.
pub struct FileAggregates {
    /// Number of files in the project.
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Payload for the `complete_multipart_upload` endpoint.
pub struct CompleteMultipartUpload {
    #[serde(flatten)]
    /// ID of the file.
    pub id: Identity,
    /// Upload ID returned by `init_multipart_upload`
    pub upload_id: String,
}
