use crate::api::resource::Resource;
use crate::dto::auth::login::*;
use crate::error::Result;

pub type Login = Resource<LoginStatus>;

impl Login {
    pub async fn status(&self) -> Result<LoginStatus> {
        let login_status_response: LoginStatusResponseWrapper = self
            .api_client
            .get::<LoginStatusResponseWrapper>("login/status")
            .await?;
        Ok(login_status_response.data)
    }
}
