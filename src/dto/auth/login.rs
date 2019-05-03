use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginStatusResponseWrapper {
  pub data: LoginStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginStatus {
  pub user: String,
  pub logged_in: bool,
  pub project: String,
  pub project_id: u64
}