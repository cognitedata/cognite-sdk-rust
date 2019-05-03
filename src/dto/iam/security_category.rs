use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecurityCategorieResponseWrapper {
  pub data: SecurityCategorieResponse,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecurityCategorieResponse {
  pub items : Vec<SecurityCategory>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecurityCategory {
  pub name : String,
  pub id : Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchSecurityCategory {
  pub add : Option<Vec<u64>>,
  pub remove : Option<Vec<u64>>,
  pub set : Option<Vec<u64>>,
}

impl From<&[u64]> for PatchSecurityCategory {
  fn from(items : &[u64]) -> PatchSecurityCategory {
      PatchSecurityCategory { 
        add : None,
        remove : None,
        set : Some(items.to_vec()),
      }
  }
}

impl From<&Option<Vec<u64>>> for PatchSecurityCategory {
  fn from(items : &Option<Vec<u64>>) -> PatchSecurityCategory {
      PatchSecurityCategory { 
        add : None,
        remove : None,
        set : Some(if items.is_none() { vec!() } else { items.clone().unwrap() }),
      }
  }
}