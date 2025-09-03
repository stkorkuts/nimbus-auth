pub mod errors;

use axum::Router;
use nimbus_auth_shared::config::AppConfig;
use tokio::net::TcpListener;

use crate::api::errors::WebApiError;

pub struct WebApi {}

impl WebApi {
    pub async fn run(_config: &AppConfig) -> Result<(), WebApiError> {
        let app = Router::new();
        let listener = TcpListener::bind("")
            .await
            .map_err(WebApiError::InvalidListenerAddr)?;
        axum::serve(listener, app)
            .await
            .map_err(WebApiError::ServeFailed)?;
        Ok(())
    }
}
