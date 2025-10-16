use axum::Router;
use nimbus_auth_shared::config::AppConfig;
use tower_http::trace::TraceLayer;

use crate::axum_api::middleware::{
    cors::apply_cors_middleware, errors::MiddlewareError, hsts::apply_hsts_middleware,
    rate_limiting::apply_rate_limiting_middleware,
};

mod cors;
pub mod errors;
mod hsts;
mod rate_limiting;

pub fn apply_middleware(mut router: Router, config: &AppConfig) -> Result<Router, MiddlewareError> {
    router = router.layer(TraceLayer::new_for_http());
    if config.use_hsts() {
        router = apply_hsts_middleware(router);
    }
    router = apply_cors_middleware(router, config.cors_origins())?;
    router = apply_rate_limiting_middleware(router);

    Ok(router)
}
