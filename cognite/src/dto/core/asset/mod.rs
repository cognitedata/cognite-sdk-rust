mod filter;

pub use self::filter::*;

use crate::dto::identity::Identity;
use crate::{CogniteExternalId, CogniteId, EqIdentity, UpdateList};
use crate::{Patch, UpdateMap, UpdateSet, UpdateSetNull};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetAggregate {
    pub child_count: Option<i32>,
    pub depth: Option<i32>,
    pub path: Option<Vec<CogniteId>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddAsset {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<CogniteExternalId>>,
}

impl From<&Asset> for AddAsset {
    fn from(asset: &Asset) -> AddAsset {
        AddAsset {
            name: asset.name.clone(),
            external_id: asset.external_id.clone(),
            parent_id: asset.parent_id,
            description: asset.description.clone(),
            metadata: asset.metadata.clone(),
            source: asset.source.clone(),
            parent_external_id: if asset.parent_id.is_none() {
                asset.parent_external_id.clone()
            } else {
                None
            },
            data_set_id: asset.data_set_id,
            labels: asset.labels.clone(),
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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UpdateSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<UpdateMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<UpdateSet<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_external_id: Option<UpdateSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<UpdateList<CogniteExternalId, CogniteExternalId>>,
}

impl From<&Asset> for Patch<PatchAsset> {
    fn from(asset: &Asset) -> Patch<PatchAsset> {
        Patch::<PatchAsset> {
            id: to_idt!(asset),
            update: PatchAsset {
                name: Some(asset.name.clone().into()),
                external_id: Some(asset.external_id.clone().into()),
                description: Some(asset.description.clone().into()),
                metadata: Some(asset.metadata.clone().into()),
                source: Some(asset.source.clone().into()),
                parent_id: asset.parent_id.map(|p| p.into()),
                parent_external_id: None,
                labels: Some(asset.labels.clone().into()),
                data_set_id: Some(asset.data_set_id.into()),
            },
        }
    }
}

impl From<&AddAsset> for PatchAsset {
    fn from(asset: &AddAsset) -> Self {
        PatchAsset {
            name: Some(asset.name.clone().into()),
            external_id: Some(asset.external_id.clone().into()),
            description: Some(asset.description.clone().into()),
            metadata: Some(asset.metadata.clone().into()),
            source: Some(asset.source.clone().into()),
            parent_id: asset.parent_id.map(|p| p.into()),
            parent_external_id: None,
            labels: Some(asset.labels.clone().into()),
            data_set_id: Some(asset.data_set_id.into()),
        }
    }
}

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
