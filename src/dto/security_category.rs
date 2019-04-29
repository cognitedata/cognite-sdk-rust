use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecurityCategory {
  pub id : u64,
  pub name : String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchSecurityCategory {
  pub add : Option<Vec<u64>>,
  pub remove : Option<Vec<u64>>,
  pub set : Option<Vec<u64>>,
}

impl From<&Vec<u64>> for PatchSecurityCategory {
  fn from(items : &Vec<u64>) -> PatchSecurityCategory {
      PatchSecurityCategory { 
        add : None,
        remove : None,
        set : Some(items.clone()),
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