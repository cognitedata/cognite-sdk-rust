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
    let login_status_response : LoginStatusResponseWrapper = self.api_client.get::<LoginStatusResponseWrapper>("login/status")?;
    Ok(login_status_response.data)
  }
}