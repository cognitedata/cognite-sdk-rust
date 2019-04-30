use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Params {
  // ASSET

  // Search
  #[serde(rename = "name")]
  AssetsSearch_Name(String),
  #[serde(rename = "description")]
  AssetsSearch_Description(String),
  #[serde(rename = "query")]
  AssetsSearch_Query(String),
  #[serde(rename = "metadata")]
  AssetsSearch_Metadata(HashMap<String, String>),
  #[serde(rename = "assetSubtrees")]
  AssetsSearch_AssetSubtrees(String),
  #[serde(rename = "minCreatedTime")]
  AssetsSearch_MinCreatedTime(u64),
  #[serde(rename = "maxCreatedTime")]
  AssetsSearch_MaxCreatedTime(u64),
  #[serde(rename = "minLastUpdatedTime")]
  AssetsSearch_MinLastUpdatedTime(u64),
  #[serde(rename = "maxLastUpdatedTime")]
  AssetsSearch_MaxLastUpdatedTime(u64),
  #[serde(rename = "sort")]
  AssetsSearch_Sort(String),
  #[serde(rename = "dir")]
  AssetsSearch_Dir(String),
  #[serde(rename = "limit")]
  AssetsSearch_Limit(u32),
  #[serde(rename = "offset")]
  AssetsSearch_Offset(u32),
  #[serde(rename = "boostName")]
  AssetsSearch_BoostName(bool),

  // ListAll
  #[serde(rename = "name")]
  AssetsListAll_Name(String),
  #[serde(rename = "fuzziness")]
  AssetsListAll_Fuzziness(u32),
  #[serde(rename = "path")]
  AssetsListAll_Path(String),
  #[serde(rename = "depth")]
  AssetsListAll_Depth(String),
  #[serde(rename = "metadata")]
  AssetsListAll_Metadata(HashMap<String,String>),
  #[serde(rename = "description")]
  AssetsListAll_Description(String),
  #[serde(rename = "source")]
  AssetsListAll_Source(String),
  #[serde(rename = "cursor")]
  AssetsListAll_Cursor(String),
  #[serde(rename = "limit")]
  AssetsListAll_Limit(u32),

  // EVENTS 

  // ListAll
  #[serde(rename = "type")]
  EventsListAll_Type(String),
  #[serde(rename = "subType")]
  EventsListAll_SubType(String),
  #[serde(rename = "assetId")]
  EventsListAll_AssetId(u64),
  #[serde(rename = "sort")]
  EventsListAll_Sort(String),
  #[serde(rename = "cursor")]
  EventsListAll_Cursor(String),
  #[serde(rename = "limit")]
  EventsListAll_Limit(u32),
  #[serde(rename = "hasDescription")]
  EventsListAll_HasDescription(bool),
  #[serde(rename = "minStartTime")]
  EventsListAll_MinStartTime(u64),
  #[serde(rename = "maxStartTime")]
  EventsListAll_MaxStartTime(u64),
  #[serde(rename = "source")]
  EventsListAll_Source(String),

  // Search
  #[serde(rename = "description")]
  EventsSearch_Description(String),
  #[serde(rename = "type")]
  EventsSearch_Type(String),
  #[serde(rename = "subType")]
  EventsSearch_SubType(String),
  #[serde(rename = "minStartTime")]
  EventsSearch_MinStartTime(u64),
  #[serde(rename = "maxStartTime")]
  EventsSearch_MaxStartTime(u64),
  #[serde(rename = "minEndTime")]
  EventsSearch_MinEndTime(u64),
  #[serde(rename = "maxEndTime")]
  EventsSearch_MaxEndTime(u64),
  #[serde(rename = "minCreatedTime")]
  EventsSearch_MinCreatedTime(u64),
  #[serde(rename = "maxCreatedTime")]
  EventsSearch_MaxCreatedTime(u64),
  #[serde(rename = "minLastUpdatedTime")]
  EventsSearch_MinLastUpdatedTime(u64),
  #[serde(rename = "maxLastUpdatedTime")]
  EventsSearch_MaxLastUpdatedTime(u64),
  #[serde(rename = "metadata")]
  EventsSearch_Metadata(HashMap<String, String>),
  #[serde(rename = "assetIds")]
  EventsSearch_AssetIds(String),
  #[serde(rename = "assetSubtrees")]
  EventsSearch_AssetSubtrees(String),
  #[serde(rename = "sort")]
  EventsSearch_Sort(String),
  #[serde(rename = "dir")]
  EventsSearch_Dir(String),
  #[serde(rename = "limit")]
  EventsSearch_Limit(u32),
  #[serde(rename = "offset")]
  EventsSearch_Offset(u32),

  // TIME SERIES

  // ListAll 
  #[serde(rename = "q")]
  TimeSeriesListAll_Q(String),
  #[serde(rename = "description")]
  TimeSeriesListAll_Description(String),
  #[serde(rename = "limit")]
  TimeSeriesListAll_Limit(u32),
  #[serde(rename = "includeMetadata")]
  TimeSeriesListAll_IncludeMetadata(bool),
  #[serde(rename = "cursor")]
  TimeSeriesListAll_Cursor(String),
  #[serde(rename = "assetId")]
  TimeSeriesListAll_AssetId(u64),
  #[serde(rename = "path")]
  TimeSeriesListAll_Path(String),

  // Retrieve
  #[serde(rename = "includeMetadata")]
  TimeSeriesRetrieve_IncludeMetadata(bool),

  // Search
  #[serde(rename = "name")]
  TimeSeriesSearch_Name(String),
  #[serde(rename = "description")]
  TimeSeriesSearch_Description(String),
  #[serde(rename = "query")]
  TimeSeriesSearch_Query(String),
  #[serde(rename = "unit")]
  TimeSeriesSearch_Unit(String),
  #[serde(rename = "isString")]
  TimeSeriesSearch_IsString(bool),
  #[serde(rename = "isStep")]
  TimeSeriesSearch_IsStep(bool),
  #[serde(rename = "metadata")]
  TimeSeriesSearch_Metadata(HashMap<String, String>),
  #[serde(rename = "assetIds")]
  TimeSeriesSearch_AssetIds(String),
  #[serde(rename = "assetSubtrees")]
  TimeSeriesSearch_AssetSubtrees(String),
  #[serde(rename = "minCreatedTime")]
  TimeSeriesSearch_MinCreatedTime(u64),
  #[serde(rename = "maxCreatedTime")]
  TimeSeriesSearch_MaxCreatedTime(u64),
  #[serde(rename = "minLastUpdatedTime")]
  TimeSeriesSearch_MinLastUpdatedTime(u64),
  #[serde(rename = "maxLastUpdatedTime")]
  TimeSeriesSearch_MaxLastUpdatedTime(u64),
  #[serde(rename = "sort")]
  TimeSeriesSearch_Sort(String),
  #[serde(rename = "dir")]
  TimeSeriesSearch_Dir(String),
  #[serde(rename = "limit")]
  TimeSeriesSearch_Limit(u32),
  #[serde(rename = "offset")]
  TimeSeriesSearch_Offset(u32),
  #[serde(rename = "boostName")]
  TimeSeriesSearch_BoostName(bool),

  // DATAPOINTS

  // Retrieve from time series
  #[serde(rename = "start")]
  DatapointsRetrieve_Start(String),
  #[serde(rename = "end")]
  DatapointsRetrieve_End(String),
  #[serde(rename = "aggregates")]
  DatapointsRetrieve_Aggregates(String),
  #[serde(rename = "granularity")]
  DatapointsRetrieve_Granularity(String),
  #[serde(rename = "limit")]
  DatapointsRetrieve_Limit(u32),
  #[serde(rename = "includeOutsidePoints")]
  DatapointsRetrieve_IncludeOutsidePoints(bool),

  #[serde(rename = "before")]
  DatapointsRetrieveLatest_Before(String),

  // Delete
  #[serde(rename = "timestamp")]
  DatapointsDelete_Timestamp(u128),
  #[serde(rename = "timestampInclusiveBegin")]
  DatapointsDelete_TimestampInclusiveBegin(u128),
  #[serde(rename = "timestampExclusiveEnd")]
  DatapointsDelete_TimestampExclusideEnd(u128),

  // FILES

  // ListAll
  #[serde(rename = "assetId")]
  FilesListAll_AssetId(String),
  #[serde(rename = "dir")]
  FilesListAll_Dir(String),
  #[serde(rename = "name")]
  FilesListAll_Name(String),
  #[serde(rename = "type")]
  FilesListAll_Type(String),
  #[serde(rename = "source")]
  FilesListAll_Source(String),
  #[serde(rename = "isUploaded")]
  FilesListAll_IsUploaded(bool),
  #[serde(rename = "limit")]
  FilesListAll_Limit(u32),
  #[serde(rename = "sort")]
  FilesListAll_Sort(u32),
  #[serde(rename = "cursor")]
  FilesListAll_Cursor(u32),

  // Search
  #[serde(rename = "name")]
  FilesSearch_Name(String),
  #[serde(rename = "directory")]
  FilesSearch_Directory(String),
  #[serde(rename = "type")]
  FilesSearch_Type(String),
  #[serde(rename = "uploaded")]
  FilesSearch_Uploaded(bool),
  #[serde(rename = "minUploadedTime")]
  FilesSearch_MinUploadedTime(u64),
  #[serde(rename = "maxUploadedTime")]
  FilesSearch_MaxUploadedTime(u64),
  #[serde(rename = "minCreatedTime")]
  FilesSearch_MinCreatedTime(u64),
  #[serde(rename = "maxCreatedTime")]
  FilesSearch_MaxCreatedTime(u64),
  #[serde(rename = "minLastUpdatedTime")]
  FilesSearch_MinLastUpdatedTime(u64),
  #[serde(rename = "maxLastUpdatedTime")]
  FilesSearch_MaxLastUpdatedTime(u64),
  #[serde(rename = "metadata")]
  FilesSearch_Metadata(HashMap<String, String>),
  #[serde(rename = "assetIds")]
  FilesSearch_AssetIds(String),
  #[serde(rename = "assetSubtrees")]
  FilesSearch_AssetSubtrees(String),
  #[serde(rename = "sort")]
  FilesSearch_Sort(String),
  #[serde(rename = "dir")]
  FilesSearch_Dir(String),
  #[serde(rename = "limit")]
  FilesSearch_Limit(u32),
  #[serde(rename = "offset")]
  FilesSearch_Offset(u32),

  // SECURITY CATEGORIES

  // List all
  #[serde(rename = "cursor")]
  SecurityCategoriesListAll_Cursor(String),
  #[serde(rename = "sort")]
  SecurityCategoriesListAll_Sort(String),
  #[serde(rename = "limit")]
  SecurityCategoriesListAll_Limit(u32),
} 