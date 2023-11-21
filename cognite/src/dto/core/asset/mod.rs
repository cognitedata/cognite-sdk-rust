mod filter;

pub use self::filter::*;

use crate::dto::identity::Identity;
use crate::{CogniteExternalId, CogniteId, EqIdentity, IntoPatch, IntoPatchItem, UpdateList};
use crate::{Patch, UpdateMap, UpdateSet, UpdateSetNull};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use super::GeoLocation;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetAggregate {
    pub child_count: Option<i32>,
    pub depth: Option<i32>,
    pub path: Option<Vec<CogniteId>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: i64,
    pub name: String,
    pub external_id: Option<String>,
    pub parent_id: Option<i64>,
    pub parent_external_id: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub source: Option<String>,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub root_id: Option<i64>,
    pub aggregates: Option<AssetAggregate>,
    pub data_set_id: Option<i64>,
    pub labels: Option<Vec<CogniteExternalId>>,
    pub geo_location: Option<GeoLocation>,
}

impl Asset {
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
pub struct AddAsset {
    pub name: String,
    pub external_id: Option<String>,
    pub parent_id: Option<i64>,
    pub description: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub source: Option<String>,
    pub parent_external_id: Option<String>,
    pub data_set_id: Option<i64>,
    pub labels: Option<Vec<CogniteExternalId>>,
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
pub struct PatchAsset {
    pub external_id: Option<UpdateSetNull<String>>,
    pub name: Option<UpdateSet<String>>,
    pub description: Option<UpdateSetNull<String>>,
    pub data_set_id: Option<UpdateSetNull<i64>>,
    pub metadata: Option<UpdateMap<String, String>>,
    pub source: Option<UpdateSetNull<String>>,
    pub parent_id: Option<UpdateSet<i64>>,
    pub parent_external_id: Option<UpdateSet<String>>,
    pub labels: Option<UpdateList<CogniteExternalId, CogniteExternalId>>,
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
pub struct RetrieveAssetsRequest {
    pub items: ::serde_json::Value,
    pub ignore_unknown_ids: bool,
    pub aggregated_properties: Option<Vec<String>>,
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
pub struct DeleteAssetsRequest {
    pub items: Vec<Identity>,
    pub ignore_unknown_ids: bool,
    pub recursive: bool,
}
