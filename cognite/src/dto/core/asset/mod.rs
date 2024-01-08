mod aggregate;
mod filter;

pub use self::aggregate::*;
pub use self::filter::*;

use crate::dto::identity::Identity;
use crate::{CogniteExternalId, CogniteId, EqIdentity, IntoPatch, IntoPatchItem, UpdateList};
use crate::{Patch, UpdateMap, UpdateSet, UpdateSetNull};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use super::common::GeoLocation;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Aggregated asset properties.
pub struct AssetAggregate {
    /// Number of direct descendants of the asset.
    pub child_count: Option<i32>,
    /// Asset path depth (number of levels below root node)
    pub depth: Option<i32>,
    /// IDs of assets on the path to the asset.
    pub path: Option<Vec<CogniteId>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// A CDF asset, representing some entity.
pub struct Asset {
    /// Server-generated ID of the asset.
    pub id: i64,
    /// Name of the asset.
    pub name: String,
    /// Unique user-provided external ID.
    pub external_id: Option<String>,
    /// ID of the parent node.
    pub parent_id: Option<i64>,
    /// External ID of the parent node.
    pub parent_external_id: Option<String>,
    /// Description of the asset.
    pub description: Option<String>,
    /// Custom, application specific metadata. String key -> String value.
    /// Limits: Maximum length of key is 128 bytes,
    /// value 10240 bytes, up to 256 key-value pairs, of total size at most 10240.
    pub metadata: Option<HashMap<String, String>>,
    /// Source of the asset.
    pub source: Option<String>,
    /// Time this asset was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this assset was last updated, in milliseconds since epoch.
    pub last_updated_time: i64,
    /// ID of the root asset.
    pub root_id: Option<i64>,
    /// Aggregated metrics computed on this asset.
    pub aggregates: Option<AssetAggregate>,
    /// ID of the data set this asset belongs to.
    pub data_set_id: Option<i64>,
    /// List of the labels associated with this asset.
    pub labels: Option<Vec<CogniteExternalId>>,
    /// Geographic metadata.
    pub geo_location: Option<GeoLocation>,
}

impl Asset {
    /// Create an asset
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the asset.
    /// * `description` - Description of the asset.
    /// * `external_id` - External ID of the asset, must be unique.
    /// * `parent_id` - ID of the parent of this asset.
    /// * `metadata` - Optional application specific metadata.
    /// * `source` - Source of this asset.
    pub fn new(
        name: &str,
        description: &str,
        external_id: Option<String>,
        parent_id: Option<i64>,
        metadata: Option<HashMap<String, String>>,
        source: Option<String>,
    ) -> Asset {
        Asset {
            name: String::from(name),
            id: 0,
            external_id,
            parent_id,
            description: Some(String::from(description)),
            metadata,
            source,
            created_time: 0,
            last_updated_time: 0,
            root_id: None,
            aggregates: None,
            data_set_id: None,
            parent_external_id: None,
            labels: None,
            geo_location: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Create an asset.
pub struct AddAsset {
    /// Name of the asset.
    pub name: String,
    /// Unique external ID of the asset.
    pub external_id: Option<String>,
    /// Parent node ID used to specify a parent-child relationship. Do not use
    /// in combination with `parent_external_id`
    pub parent_id: Option<i64>,
    /// Description of the asset.
    pub description: Option<String>,
    /// Custom, application specific metadata. String key -> String value.
    /// Limits: Maximum length of key is 128 bytes, value 10240 bytes,
    /// up to 256 key-value pairs, of total size at most 10240.
    pub metadata: Option<HashMap<String, String>>,
    /// Source of the asset.
    pub source: Option<String>,
    /// External ID of the parent asset.
    ///  When specifying this field, the API will resolve the external ID
    /// into an internal ID and use the internal ID to store the
    /// parent-child relation. As a result, a later change to update the parent's
    /// external ID will not affect this parent-child relationship as
    /// it is based on internal ID.
    ///
    /// Do not use in combination with `parent_id`
    pub parent_external_id: Option<String>,
    /// ID of the dataset this asset belongs to.
    pub data_set_id: Option<i64>,
    /// A list of labels associated with this asset.
    pub labels: Option<Vec<CogniteExternalId>>,
    /// Geographic metadata.
    pub geo_location: Option<GeoLocation>,
}

impl From<Asset> for AddAsset {
    fn from(asset: Asset) -> AddAsset {
        AddAsset {
            name: asset.name,
            external_id: asset.external_id,
            parent_id: asset.parent_id,
            description: asset.description,
            metadata: asset.metadata,
            source: asset.source,
            parent_external_id: if asset.parent_id.is_none() {
                asset.parent_external_id
            } else {
                None
            },
            data_set_id: asset.data_set_id,
            labels: asset.labels,
            geo_location: asset.geo_location,
        }
    }
}

impl EqIdentity for AddAsset {
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
/// Update an asset.
pub struct PatchAsset {
    /// Unique external ID of the asset.
    pub external_id: Option<UpdateSetNull<String>>,
    /// Name of the asset.
    pub name: Option<UpdateSet<String>>,
    /// Description of the asset.
    pub description: Option<UpdateSetNull<String>>,
    /// ID of the dataset this asset belongs to.
    pub data_set_id: Option<UpdateSetNull<i64>>,
    /// Custom, application specific metadata. String key -> String value.
    /// Limits: Maximum length of key is 128 bytes, value 10240 bytes,
    /// up to 256 key-value pairs, of total size at most 10240.
    pub metadata: Option<UpdateMap<String, String>>,
    /// Source of the asset.
    pub source: Option<UpdateSetNull<String>>,
    /// Parent node ID used to specify a parent-child relationship. Do not use
    /// in combination with `parent_external_id`
    pub parent_id: Option<UpdateSet<i64>>,
    /// External ID of the parent asset.
    ///  When specifying this field, the API will resolve the external ID
    /// into an internal ID and use the internal ID to store the
    /// parent-child relation. As a result, a later change to update the parent's
    /// external ID will not affect this parent-child relationship as
    /// it is based on internal ID.
    ///
    /// Do not use in combination with `parent_id`
    pub parent_external_id: Option<UpdateSet<String>>,
    /// A list of labels associated with this asset.
    pub labels: Option<UpdateList<CogniteExternalId, CogniteExternalId>>,
    /// Geographic metadata.
    pub geo_location: Option<UpdateSetNull<GeoLocation>>,
}

impl IntoPatch<Patch<PatchAsset>> for Asset {
    fn patch(self, ignore_nulls: bool) -> Patch<PatchAsset> {
        Patch::<PatchAsset> {
            id: to_idt!(self),
            update: PatchAsset {
                external_id: self.external_id.patch(ignore_nulls),
                name: self.name.patch(ignore_nulls),
                description: self.description.patch(ignore_nulls),
                data_set_id: self.data_set_id.patch(ignore_nulls),
                metadata: self.metadata.patch(ignore_nulls),
                source: self.source.patch(ignore_nulls),
                parent_id: self.parent_id.and_then(|p| p.patch(ignore_nulls)),
                parent_external_id: None,
                labels: self.labels.patch(ignore_nulls),
                geo_location: self.geo_location.patch(ignore_nulls),
            },
        }
    }
}

impl IntoPatch<PatchAsset> for AddAsset {
    fn patch(self, ignore_nulls: bool) -> PatchAsset {
        let mut parent_id = None;
        let mut parent_external_id = None;
        if let Some(p_id) = self.parent_id {
            parent_id = p_id.patch(ignore_nulls);
        } else if let Some(p_extid) = self.parent_external_id {
            parent_external_id = p_extid.patch(ignore_nulls);
        }
        PatchAsset {
            external_id: self.external_id.patch(ignore_nulls),
            name: self.name.patch(ignore_nulls),
            description: self.description.patch(ignore_nulls),
            data_set_id: self.data_set_id.patch(ignore_nulls),
            metadata: self.metadata.patch(ignore_nulls),
            source: self.source.patch(ignore_nulls),
            parent_id,
            parent_external_id,
            labels: self.labels.patch(ignore_nulls),
            geo_location: self.geo_location.patch(ignore_nulls),
        }
    }
}

impl From<Asset> for Patch<PatchAsset> {
    fn from(value: Asset) -> Self {
        IntoPatch::<Patch<PatchAsset>>::patch(value, false)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetrieveAssetsRequest {
    pub items: ::serde_json::Value,
    pub ignore_unknown_ids: bool,
    pub aggregated_properties: Option<Vec<AssetAggregatedProperty>>,
}

impl<T: Serialize> From<&Vec<T>> for RetrieveAssetsRequest {
    fn from(items: &Vec<T>) -> RetrieveAssetsRequest {
        RetrieveAssetsRequest {
            items: json!(items),
            ignore_unknown_ids: true,
            aggregated_properties: None,
        }
    }
}

impl<T: Serialize> From<&[T]> for RetrieveAssetsRequest {
    fn from(items: &[T]) -> RetrieveAssetsRequest {
        RetrieveAssetsRequest {
            items: json!(items),
            ignore_unknown_ids: true,
            aggregated_properties: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Request for deleting a list of assets.
pub struct DeleteAssetsRequest {
    /// Internal or external IDs of assets to delete.
    pub items: Vec<Identity>,
    /// If `true`, ignore any IDs that do not exist in CDF.
    pub ignore_unknown_ids: bool,
    /// If `true`, recursively delete any children of the deleted assets.
    pub recursive: bool,
}
