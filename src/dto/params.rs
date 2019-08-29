use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Params {
  // ASSET

  // List
  #[serde(rename = "limit")]
  AssetsList_Limit(u32),
  #[serde(rename = "cursor")]
  AssetsList_Cursor(String),
  #[serde(rename = "name")]
  AssetsList_Name(String),
  #[serde(rename = "parentIds")]
  AssetsList_ParentIds(Vec<u64>),
  #[serde(rename = "rootIds")]
  AssetsList_RootIds(Vec<u64>),
  #[serde(rename = "source")]
  AssetsList_Source(String),
  #[serde(rename = "root")]
  AssetsList_Root(String),
  #[serde(rename = "minCreatedTime")]
  AssetsList_MinCreatedTime(u64),
  #[serde(rename = "maxCreatedTime")]
  AssetsList_MaxCreatedTime(u64),
  #[serde(rename = "minLastUpdatedTime")]
  AssetsList_MinLastUpdatedTime(u64),
  #[serde(rename = "maxLastUpdatedTime")]
  AssetsList_MaxLastUpdatedTime(u64),
  #[serde(rename = "externalIdPrefix")]
  AssetsList_ExternalIdPrefix(String),

  // EVENTS 

  // Filter events
  #[serde(rename = "limit")]
  EventsFilter_Limit(u32),
  #[serde(rename = "cursor")]
  EventsFilter_Cursor(String),
  #[serde(rename = "minStartTime")]
  EventsFilter_MinStartTime(u64),
  #[serde(rename = "maxStartTime")]
  EventsFilter_MaxStartTime(u64),
  #[serde(rename = "minEndTime")]
  EventsFilter_MinEndTime(u64),
  #[serde(rename = "maxEndTime")]
  EventsFilter_MaxEndTime(u64),
  #[serde(rename = "assetIds")]
  EventsFilter_AssetIds(String),
  #[serde(rename = "rootIds")]
  EventsFilter_RootIds(String),
  #[serde(rename = "source")]
  EventsFilter_Source(String),
  #[serde(rename = "type")]
  EventsFilter_Type(String),
  #[serde(rename = "subType")]
  EventsFilter_SubType(String),
  #[serde(rename = "minCreatedTime")]
  EventsFilter_MinCreatedTime(u64),
  #[serde(rename = "maxCreatedTime")]
  EventsFilter_MaxCreatedTime(u64),
  #[serde(rename = "minLastUpdatedTime")]
  EventsFilter_MinLastUpdatedTime(u64),
  #[serde(rename = "maxLastUpdatedTime")]
  EventsFilter_MaxLastUpdatedTime(u64),
  #[serde(rename = "externalIdPrefix")]
  EventsFilter_ExternalIdPrefix(String),

  // TIME SERIES

  // ListAll 
  #[serde(rename = "limit")]
  TimeSeriesListAll_Limit(u32),
  #[serde(rename = "includeMetadata")]
  TimeSeriesListAll_IncludeMetadata(bool),
  #[serde(rename = "cursor")]
  TimeSeriesListAll_Cursor(String),
  #[serde(rename = "assetIds")]
  TimeSeriesListAll_AssetIds(Vec<u64>),

  // FILES

  // List files
  #[serde(rename = "limit")]
  FilesList_Limit(u32),
  #[serde(rename = "cursor")]
  FilesList_Cursor(String),
  #[serde(rename = "name")]
  FilesList_Name(String),
  #[serde(rename = "mimeType")]
  FilesList_MimeType(String),
  #[serde(rename = "source")]
  FilesList_Source(u32),
  #[serde(rename = "assetIds")]
  FilesList_AssetIds(String),
  #[serde(rename = "minCreatedTime")]
  FilesList_MinCreatedTime(u64),
  #[serde(rename = "maxCreatedTime")]
  FilesList_MaxCreatedTime(u64),
  #[serde(rename = "minLastUpdatedTime")]
  FilesList_MinLastUpdatedTime(u64),
  #[serde(rename = "maxLastUpdatedTime")]
  FilesList_MaxLastUpdatedTime(u64),
  #[serde(rename = "minUploadedTime")]
  FilesList_MinUploadedTime(u64),
  #[serde(rename = "maxUploadedTime")]
  FilesList_MaxUploadedTime(u64),
  #[serde(rename = "externalIdPrefix")]
  FilesList_ExternalIdPrefix(String),
  #[serde(rename = "uploaded")]
  FilesList_Uploaded(bool),

  // SECURITY CATEGORIES

  // List all
  #[serde(rename = "cursor")]
  SecurityCategoriesListAll_Cursor(String),
  #[serde(rename = "sort")]
  SecurityCategoriesListAll_Sort(String),
  #[serde(rename = "limit")]
  SecurityCategoriesListAll_Limit(u32),

  // API KEYS

  // List all
  #[serde(rename = "all")]
  ApiKeysListAll_All(bool),
  #[serde(rename = "serviceAccountId")]
  ApiKeysListAll_ServiceAccountId(u64),
  #[serde(rename = "includeDeleted")]
  ApiKeysListAll_IncludeDeleted(bool),

   // GGROUPS

  // List 
  #[serde(rename = "all")]
  GroupsList_All(bool),
} 
