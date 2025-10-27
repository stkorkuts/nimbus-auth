use axum::Router;
use nimbus_auth_shared::config::AppConfig;

use crate::web_api::middleware::{
    cors::apply_cors_middleware, errors::MiddlewareError, hsts::apply_hsts_middleware,
    rate_limiting::apply_rate_limiting_middleware, tracing::apply_tracing_middleware,
};

mod cors;
pub mod errors;
mod hsts;
mod rate_limiting;
mod tracing;

pub fn apply_middleware(mut router: Router, config: &AppConfig) -> Result<Router, MiddlewareError> {
    if config.use_hsts() {
        router = apply_hsts_middleware(router);
    }
    router = apply_cors_middleware(router, config.cors_origins())?;
    router = apply_rate_limiting_middleware(router);
    router = apply_tracing_middleware(router);

    Ok(router)
}
