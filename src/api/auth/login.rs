use crate::api::ApiClient;
use crate::dto::auth::login::*;
use crate::error::Result;

pub struct Login {
    api_client: ApiClient,
}

impl Login {
    pub fn new(api_client: ApiClient) -> Login {
        Login { api_client }
    }

    pub async fn status(&self) -> Result<LoginStatus> {
        let login_status_response: LoginStatusResponseWrapper = self
            .api_client
            .get::<LoginStatusResponseWrapper>("login/status")
            .await?;
        Ok(login_status_response.data)
    }
}
