use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchItem {
  #[serde(skip_serializing_if = "::serde_json::Value::is_null")]
  pub set : ::serde_json::Value,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub set_null : Option<bool>,
}

impl From<&String> for PatchItem {
  fn from(item : &String) -> PatchItem {
      PatchItem { 
        set : json!(item), 
        set_null : None
      }
  }
}

impl From<bool> for PatchItem {
  fn from(item : bool) -> PatchItem {
      PatchItem { 
        set : json!(item), 
        set_null : None
      }
  }
}

impl From<&Option<String>> for PatchItem {
  fn from(item : &Option<String>) -> PatchItem {
      PatchItem { 
        set : json!(item), 
        set_null : if item.is_none() { Some(item.is_none()) } else { None }
      }
  }
}

impl From<&Option<u64>> for PatchItem {
  fn from(item : &Option<u64>) -> PatchItem {
      PatchItem { 
        set : json!(item), 
        set_null : if item.is_none() { Some(item.is_none()) } else { None }
      }
  }
}

impl From<&Option<Vec<u64>>> for PatchItem {
  fn from(item : &Option<Vec<u64>>) -> PatchItem {
      PatchItem { 
        set : json!(item), 
        set_null : if item.is_none() { Some(item.is_none()) } else { None }
      }
  }
}

impl From<&Option<HashMap<String, String>>> for PatchItem {
  fn from(item : &Option<HashMap<String, String>>) -> PatchItem {
      PatchItem { 
        set : if item.is_none() { json!(HashMap::<String,String>::new())} else { json!(item) }, 
        set_null : None
      }
  }
}