use axum::{
    Router,
    routing::{get, post},
};
use nimbus_auth_application::use_cases::UseCases;
use nimbus_auth_shared::config::AppConfig;
use tokio::{net::TcpListener, sync::oneshot};

use crate::web_api::{
    errors::WebApiError,
    handlers::{
        get_public_key::{handle_get_active_public_key, handle_get_public_key_by_id},
        refresh::handle_refresh,
        rotate_keypairs::handle_rotate_keypairs,
        signin::handle_signin,
        signup::handle_signup,
    },
    middleware::apply_middleware,
};

pub mod errors;
mod extractors;
mod handlers;
mod middleware;
mod responses;

pub struct WebApi {}

impl WebApi {
    pub async fn serve(
        config: &AppConfig,
        use_cases: UseCases,
        shutdown_signal_receiver: oneshot::Receiver<()>,
    ) -> Result<(), WebApiError> {
        let (shutdown_result_sender, shutdown_result_receiver) =
            oneshot::channel::<Result<(), WebApiError>>();

        let mut router = Router::new()
            .route("/keypairs/rotate", post(handle_rotate_keypairs))
            .route("/public_keys/active", get(handle_get_active_public_key))
            .route(
                "/public_keys/by_id/:key_id",
                get(handle_get_public_key_by_id),
            )
            .route("/auth/signup", post(handle_signup))
            .route("/auth/signin", post(handle_signin))
            .route("/auth/refresh", post(handle_refresh))
            .with_state(use_cases);

        router = apply_middleware(router, config)?;

        let listener = TcpListener::bind(config.server_addr())
            .await
            .map_err(WebApiError::InvalidListenerAddr)?;

        axum::serve(listener, router)
            .with_graceful_shutdown(async {
                shutdown_result_sender
                    .send(
                        shutdown_signal_receiver
                            .await
                            .map_err(|err| WebApiError::from(err)),
                    )
                    .unwrap();
            })
            .await
            .map_err(WebApiError::ServeFailed)?;

        shutdown_result_receiver.await.unwrap()?;
        Ok(())
    }
}
