mod filter;
mod synthetic;

pub use self::filter::*;
pub use self::synthetic::*;

use crate::models::instances::CogniteTimeseries;
use crate::IntoPatch;
use crate::IntoPatchItem;
use crate::UpsertOptions;
use crate::{EqIdentity, Identity, Patch, UpdateList, UpdateMap, UpdateSet, UpdateSetNull};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// A CDF time series.
pub struct TimeSeries {
    /// Time series internal ID.
    pub id: i64,
    /// Time series external ID. Must be unique for all time series in the project.
    pub external_id: Option<String>,
    /// Time series name.
    pub name: Option<String>,
    /// Whether this is a time series for string or double data points.
    pub is_string: bool,
    /// Custom, application specific metadata. String key -> String value.
    /// Maximum length of key is 128 bytes, up to 256 key-value pairs,
    /// of total size of at most 10000 bytes across all keys and values.
    pub metadata: Option<HashMap<String, String>>,
    /// The physical unit of the time series (free-text field).
    pub unit: Option<String>,
    /// The physical unit of the time series as represented in the unit catalog.
    pub unit_external_id: Option<String>,
    /// Asset ID of equipment linked to this time series.
    pub asset_id: Option<i64>,
    /// Whether this is a step time series or not.
    pub is_step: bool,
    /// Description of the time series.
    pub description: Option<String>,
    /// The required security categories to access this time series.
    pub security_categories: Option<Vec<i64>>,
    /// Time this time series was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this time series was last updated, in milliseconds since epoch.
    pub last_updated_time: i64,
    /// Data set this time series belongs to.
    pub data_set_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Create a new time series.
pub struct AddTimeSeries {
    /// Time series external ID. Must be unique accross all time series in the project.
    pub external_id: Option<String>,
    /// Time series name.
    pub name: Option<String>,
    /// Whether this is a time series for string or double data points.
    pub is_string: bool,
    /// Custom, application specific metadata. String key -> String value.
    /// Maximum length of key is 128 bytes, up to 256 key-value pairs,
    /// of total size of at most 10000 bytes across all keys and values.
    pub metadata: Option<HashMap<String, String>>,
    /// The physical unit of the time series (free-text field).
    pub unit: Option<String>,
    /// The physical unit of the time series as represented in the unit catalog.
    pub unit_external_id: Option<String>,
    /// ID of the asset this time series belongs to.
    pub asset_id: Option<i64>,
    /// Whether this is a step time series or not.
    pub is_step: bool,
    /// Description of the time series.
    pub description: Option<String>,
    /// The required security categories to access this time series.
    pub security_categories: Option<Vec<i64>>,
    /// Data set this time series belongs to.
    pub data_set_id: Option<i64>,
}

/// Add Core DM or classic time series.
pub enum AddDmOrTimeSeries<'a> {
    /// Classic time series.
    TimeSeries(&'a [AddTimeSeries]),
    /// Core DM timeseries
    Cdm(&'a [CogniteTimeseries]),
}

impl From<TimeSeries> for AddTimeSeries {
    fn from(time_serie: TimeSeries) -> AddTimeSeries {
        AddTimeSeries {
            name: time_serie.name,
            external_id: time_serie.external_id,
            is_string: time_serie.is_string,
            metadata: time_serie.metadata,
            unit: time_serie.unit,
            unit_external_id: time_serie.unit_external_id,
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
            _ => false,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Update a time series.
pub struct PatchTimeSeries {
    /// Time series name.
    pub name: Option<UpdateSetNull<String>>,
    /// Time series external ID. Must be unique accross all time series in the project.
    pub external_id: Option<UpdateSetNull<String>>,
    /// Custom, application specific metadata. String key -> String value.
    /// Maximum length of key is 128 bytes, up to 256 key-value pairs,
    /// of total size of at most 10000 bytes across all keys and values.
    pub metadata: Option<UpdateMap<String, String>>,
    /// The physical unit of the time series (free-text field).
    pub unit: Option<UpdateSetNull<String>>,
    /// The physical unit of the time series as represented in the unit catalog.
    pub unit_external_id: Option<UpdateSetNull<String>>,
    /// ID of the asset this time series belongs to.
    pub asset_id: Option<UpdateSetNull<i64>>,
    /// Description of the time series.
    pub description: Option<UpdateSetNull<String>>,
    /// The required security categories to access this time series.
    pub security_categories: Option<UpdateList<i64, i64>>,
    /// Data set this time series belongs to.
    pub data_set_id: Option<UpdateSetNull<i64>>,
    /// Whether this is a step time series or not.
    pub is_step: Option<UpdateSet<bool>>,
}

impl IntoPatch<Patch<PatchTimeSeries>> for TimeSeries {
    fn patch(self, options: &UpsertOptions) -> Patch<PatchTimeSeries> {
        Patch::<PatchTimeSeries> {
            id: to_idt!(self),
            update: PatchTimeSeries {
                name: self.name.patch(options),
                external_id: self.external_id.patch(options),
                metadata: self.metadata.patch(options),
                unit: self.unit.patch(options),
                unit_external_id: self.unit_external_id.patch(options),
                asset_id: self.asset_id.patch(options),
                description: self.description.patch(options),
                security_categories: self.security_categories.patch(options),
                data_set_id: self.data_set_id.patch(options),
                is_step: self.is_step.patch(options),
            },
        }
    }
}

impl IntoPatch<PatchTimeSeries> for AddTimeSeries {
    fn patch(self, options: &UpsertOptions) -> PatchTimeSeries {
        PatchTimeSeries {
            name: self.name.patch(options),
            external_id: self.external_id.patch(options),
            metadata: self.metadata.patch(options),
            unit: self.unit.patch(options),
            unit_external_id: self.unit_external_id.patch(options),
            asset_id: self.asset_id.patch(options),
            description: self.description.patch(options),
            security_categories: self.security_categories.patch(options),
            data_set_id: self.data_set_id.patch(options),
            is_step: self.is_step.patch(options),
        }
    }
}

impl From<TimeSeries> for Patch<PatchTimeSeries> {
    fn from(time_serie: TimeSeries) -> Patch<PatchTimeSeries> {
        IntoPatch::<Patch<PatchTimeSeries>>::patch(time_serie, &Default::default())
    }
}
