mod filter;

pub use self::filter::*;

use crate::{EqIdentity, Identity, Patch, UpdateList, UpdateMap, UpdateSet, UpdateSetNull};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerie {
    pub id: i64,
    pub external_id: Option<String>,
    pub name: Option<String>,
    pub is_string: bool,
    pub metadata: Option<HashMap<String, String>>,
    pub unit: Option<String>,
    pub asset_id: Option<i64>,
    pub is_step: bool,
    pub description: Option<String>,
    pub security_categories: Option<Vec<i64>>,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub data_set_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddTimeSerie {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub is_string: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    pub is_step: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_categories: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<i64>,
}

impl From<&TimeSerie> for AddTimeSerie {
    fn from(time_serie: &TimeSerie) -> AddTimeSerie {
        AddTimeSerie {
            name: time_serie.name.clone(),
            external_id: time_serie.external_id.clone(),
            is_string: time_serie.is_string,
            metadata: time_serie.metadata.clone(),
            unit: time_serie.unit.clone(),
            asset_id: time_serie.asset_id,
            is_step: time_serie.is_step,
            description: time_serie.description.clone(),
            security_categories: time_serie.security_categories.clone(),
            data_set_id: time_serie.data_set_id,
        }
    }
}

impl EqIdentity for AddTimeSerie {
    fn eq(&self, id: &Identity) -> bool {
        match id {
            Identity::Id { id: _ } => false,
            Identity::ExternalId { external_id } => self.external_id.as_ref() == Some(external_id),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchTimeSerie {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<UpdateMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<UpdateSetNull<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_categories: Option<UpdateList<i64, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_step: Option<UpdateSet<bool>>,
}

impl From<&TimeSerie> for Patch<PatchTimeSerie> {
    fn from(time_serie: &TimeSerie) -> Patch<PatchTimeSerie> {
        Patch::<PatchTimeSerie> {
            id: to_idt!(time_serie),
            update: PatchTimeSerie {
                name: Some(time_serie.name.clone().into()),
                external_id: Some(time_serie.external_id.clone().into()),
                metadata: Some(time_serie.metadata.clone().into()),
                unit: Some(time_serie.unit.clone().into()),
                asset_id: Some(time_serie.asset_id.into()),
                description: Some(time_serie.description.clone().into()),
                security_categories: Some(time_serie.security_categories.clone().into()),
                data_set_id: Some(time_serie.data_set_id.into()),
                is_step: Some(time_serie.is_step.into()),
            },
        }
    }
}

impl From<&AddTimeSerie> for PatchTimeSerie {
    fn from(time_serie: &AddTimeSerie) -> Self {
        PatchTimeSerie {
            name: Some(time_serie.name.clone().into()),
            external_id: Some(time_serie.external_id.clone().into()),
            metadata: Some(time_serie.metadata.clone().into()),
            unit: Some(time_serie.unit.clone().into()),
            asset_id: Some(time_serie.asset_id.into()),
            description: Some(time_serie.description.clone().into()),
            security_categories: Some(time_serie.security_categories.clone().into()),
            data_set_id: Some(time_serie.data_set_id.into()),
            is_step: Some(time_serie.is_step.into()),
        }
    }
}
