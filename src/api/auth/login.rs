use crate::api::ApiClient;
use crate::error::{Result};
use crate::dto::auth::login::*;

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
    match self.api_client.get::<LoginStatusResponseWrapper>("login/status") {
      Ok(login_status_response) => {
        let status = login_status_response.data;
        Ok(status)
      },
      Err(e) => Err(e)
    }
  }
}