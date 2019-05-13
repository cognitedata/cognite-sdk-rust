use crate::dto::patch_item::PatchItem;
use crate::dto::iam::security_category::PatchSecurityCategory;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerieListResponse {
  pub items : Vec<TimeSerie>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerie {
  pub id: u64,
  pub external_id : Option<String>,
  pub name: String,
  pub is_string: bool,
  pub metadata: Option<HashMap<String, String>>,
  pub unit: Option<String>,
  pub asset_id: Option<u64>,
  pub is_step: bool,
  pub description: String,
  pub security_categories: Option<Vec<u64>>,
  pub created_time: u128,
  pub last_updated_time: u128
}

impl TimeSerie {
  pub fn new(name: &str,
              external_id: Option<String>,
              is_string: bool,
              metadata : Option<HashMap<String, String>>,
              unit: Option<String>,
              asset_id: Option<u64>,
              is_step: bool,
              description: &str,
              security_categories: Option<Vec<u64>>) -> TimeSerie {
    TimeSerie {
      name: String::from(name),
      external_id: external_id,
      is_string: is_string,
      metadata : metadata,
      unit: unit,
      asset_id: asset_id,
      is_step: is_step,
      description: String::from(description),
      security_categories: security_categories,
      id: 0,
      created_time: 0,
      last_updated_time: 0
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchTimeSerie {
  id : u64,
  update : PatchTimeSerieFields,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PatchTimeSerieFields {
  name : PatchItem,
  external_id : PatchItem,
  //metadata : PatchItem,
  unit : PatchItem,
  asset_id : PatchItem,
  description : PatchItem,
  security_categories : PatchSecurityCategory,
  is_string : PatchItem,
  is_step : PatchItem,
}

impl From<&TimeSerie> for PatchTimeSerie {
  fn from(time_serie : &TimeSerie) -> PatchTimeSerie {
    PatchTimeSerie {
      id : time_serie.id,
      update : PatchTimeSerieFields {
        name : PatchItem::from(&time_serie.name),
        external_id : PatchItem::from(&time_serie.external_id),
        //metadata : PatchItem::from(&time_serie.metadata),
        unit : PatchItem::from(&time_serie.unit),
        asset_id : PatchItem::from(&time_serie.asset_id),
        description : PatchItem::from(&time_serie.description),
        security_categories : PatchSecurityCategory::from(&time_serie.security_categories),
        is_string : PatchItem::from(time_serie.is_string),
        is_step : PatchItem::from(time_serie.is_step),
      }
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerieId {
  id : u64
}

impl From<&TimeSerie> for TimeSerieId {
  fn from(time_serie : &TimeSerie) -> TimeSerieId {
    TimeSerieId {
      id : time_serie.id
    }
  }
}

impl From<u64> for TimeSerieId {
  fn from(time_serie_id : u64) -> TimeSerieId {
    TimeSerieId {
      id : time_serie_id
    }
  }
}