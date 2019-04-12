use super::{ApiClient};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginStatusResponse {
  data: LoginStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginStatus {
  pub user: String,
  pub logged_in: bool,
  pub project: String,
  pub project_id: u64
}

pub struct Login {
  api_client : ApiClient,
}

impl Login {
  pub fn new(api_client : ApiClient) -> Login {
    Login {
      api_client : api_client
    }
  }

  pub fn status(&self) -> LoginStatus {
    let login_status_response_json = self.api_client.get("login/status".to_string()).unwrap();
    let login_status_response : LoginStatusResponse = serde_json::from_str(&login_status_response_json).unwrap();
    let status = login_status_response.data;
    status
  }
}