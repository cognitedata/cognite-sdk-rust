use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchItem {
  pub set : ::serde_json::Value,
  pub set_null : bool,
}

impl From<&String> for PatchItem {
  fn from(item : &String) -> PatchItem {
      PatchItem { 
        set : json!(item), 
        set_null : false
      }
  }
}

impl From<bool> for PatchItem {
  fn from(item : bool) -> PatchItem {
      PatchItem { 
        set : json!(item), 
        set_null : false
      }
  }
}

impl From<&Option<String>> for PatchItem {
  fn from(item : &Option<String>) -> PatchItem {
      PatchItem { 
        set : json!(item), 
        set_null : item.is_none() 
      }
  }
}

impl From<&Option<u64>> for PatchItem {
  fn from(item : &Option<u64>) -> PatchItem {
      PatchItem { 
        set : json!(item), 
        set_null : item.is_none() 
      }
  }
}

impl From<&Option<Vec<u64>>> for PatchItem {
  fn from(item : &Option<Vec<u64>>) -> PatchItem {
      PatchItem { 
        set : json!(item), 
        set_null : item.is_none() 
      }
  }
}

impl From<&Option<HashMap<String, String>>> for PatchItem {
  fn from(item : &Option<HashMap<String, String>>) -> PatchItem {
      PatchItem { 
        set : json!(item), 
        set_null : item.is_none() 
      }
  }
}