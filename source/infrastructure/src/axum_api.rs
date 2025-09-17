use axum::{
    Router,
    routing::{get, post},
};
use nimbus_auth_application::use_cases::UseCases;
use nimbus_auth_shared::config::AppConfig;
use tokio::net::TcpListener;

use crate::axum_api::{
    errors::WebApiError,
    handlers::{
        get_public_key::handle_get_public_key, refresh::handle_refresh,
        rotate_keypairs::handle_rotate_keypairs, signin::handle_signin, signup::handle_signup,
    },
};

pub mod errors;
mod handlers;

pub struct WebApi {}

impl WebApi {
    pub async fn run(config: &AppConfig, use_cases: UseCases) -> Result<(), WebApiError> {
        let app = Router::new()
            .route("rotate_keypairs", post(handle_rotate_keypairs))
            .route("get_public_key", get(handle_get_public_key))
            .route("signup", post(handle_signup))
            .route("signin", post(handle_signin))
            .route("refresh", post(handle_refresh))
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
