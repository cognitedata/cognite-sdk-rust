use crate::api::ApiClient;
use crate::error::{Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginStatusResponseWrapper {
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

  pub fn status(&self) -> Result<LoginStatus> {
    let http_params = None;
    match self.api_client.get::<LoginStatusResponseWrapper>("login/status", http_params) {
      Ok(login_status_response) => {
        let status = login_status_response.data;
        Ok(status)
      },
      Err(e) => Err(e)
    }
  }
}