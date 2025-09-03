use axum::{Router, routing::post};
use nimbus_auth_application::use_cases::UseCases;
use nimbus_auth_shared::config::AppConfig;
use tokio::net::TcpListener;

use crate::api::{errors::WebApiError, handlers::signup::handle_signup};

pub mod errors;
mod handlers;

pub struct WebApi {}

impl WebApi {
    pub async fn run(config: &AppConfig, use_cases: UseCases) -> Result<(), WebApiError> {
        let app = Router::new()
            .route("signup", post(handle_signup))
            .with_state(use_cases);
        let listener = TcpListener::bind(config.server_addr())
            .await
            .map_err(WebApiError::InvalidListenerAddr)?;
        axum::serve(listener, app)
            .await
            .map_err(WebApiError::ServeFailed)?;
        Ok(())
    }
}
