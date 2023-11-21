mod filter;
mod synthetic;

pub use self::filter::*;
pub use self::synthetic::*;

use crate::{EqIdentity, Identity, Patch, UpdateList, UpdateMap, UpdateSet, UpdateSetNull};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeries {
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddTimeSeries {
    pub external_id: Option<String>,
    pub name: Option<String>,
    pub is_string: bool,
    pub metadata: Option<HashMap<String, String>>,
    pub unit: Option<String>,
    pub asset_id: Option<i64>,
    pub is_step: bool,
    pub description: Option<String>,
    pub security_categories: Option<Vec<i64>>,
    pub data_set_id: Option<i64>,
}

impl From<TimeSeries> for AddTimeSeries {
    fn from(time_serie: TimeSeries) -> AddTimeSeries {
        AddTimeSeries {
            name: time_serie.name,
            external_id: time_serie.external_id,
            is_string: time_serie.is_string,
            metadata: time_serie.metadata,
            unit: time_serie.unit,
            asset_id: time_serie.asset_id,
            is_step: time_serie.is_step,
            description: time_serie.description,
            security_categories: time_serie.security_categories,
            data_set_id: time_serie.data_set_id,
        }
    }
}

impl EqIdentity for AddTimeSeries {
    fn eq(&self, id: &Identity) -> bool {
        match id {
            Identity::Id { id: _ } => false,
            Identity::ExternalId { external_id } => self.external_id.as_ref() == Some(external_id),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchTimeSerie {
    pub name: Option<UpdateSetNull<String>>,
    pub external_id: Option<UpdateSetNull<String>>,
    pub metadata: Option<UpdateMap<String, String>>,
    pub unit: Option<UpdateSetNull<String>>,
    pub asset_id: Option<UpdateSetNull<i64>>,
    pub description: Option<UpdateSetNull<String>>,
    pub security_categories: Option<UpdateList<i64, i64>>,
    pub data_set_id: Option<UpdateSetNull<i64>>,
    pub is_step: Option<UpdateSet<bool>>,
}

impl From<TimeSeries> for Patch<PatchTimeSerie> {
    fn from(time_serie: TimeSeries) -> Patch<PatchTimeSerie> {
        Patch::<PatchTimeSerie> {
            id: to_idt!(time_serie),
            update: PatchTimeSerie {
                name: Some(time_serie.name.into()),
                external_id: Some(time_serie.external_id.into()),
                metadata: Some(time_serie.metadata.into()),
                unit: Some(time_serie.unit.into()),
                asset_id: Some(time_serie.asset_id.into()),
                description: Some(time_serie.description.into()),
                security_categories: Some(time_serie.security_categories.into()),
                data_set_id: Some(time_serie.data_set_id.into()),
                is_step: Some(time_serie.is_step.into()),
            },
        }
    }
}

impl From<AddTimeSeries> for PatchTimeSerie {
    fn from(time_serie: AddTimeSeries) -> Self {
        PatchTimeSerie {
            name: Some(time_serie.name.into()),
            external_id: Some(time_serie.external_id.into()),
            metadata: Some(time_serie.metadata.into()),
            unit: Some(time_serie.unit.into()),
            asset_id: Some(time_serie.asset_id.into()),
            description: Some(time_serie.description.into()),
            security_categories: Some(time_serie.security_categories.into()),
            data_set_id: Some(time_serie.data_set_id.into()),
            is_step: Some(time_serie.is_step.into()),
        }
    }
}
