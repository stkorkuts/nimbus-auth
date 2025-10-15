use std::str::FromStr;

use axum::{
    Router,
    http::{HeaderName, HeaderValue, Method},
};
use nimbus_auth_shared::config::AppConfig;
use tower_http::{
    cors::{Any, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

use crate::axum_api::middleware::{
    cors::apply_cors_middleware, errors::MiddlewareError, hsts::apply_hsts_middleware,
};

mod cors;
pub mod errors;
mod hsts;
mod rate_limiting;

pub fn apply_middleware(mut router: Router, config: &AppConfig) -> Result<Router, MiddlewareError> {
    router = router.layer(TraceLayer::new_for_http());
    router = apply_hsts_middleware(router);
    router = apply_cors_middleware(router, config.cors_origins())?;

    Ok(router)
}
