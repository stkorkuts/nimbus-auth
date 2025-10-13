use std::str::FromStr;

use axum::{
    Router,
    http::{HeaderName, HeaderValue, Method},
};
use nimbus_auth_shared::config::AppConfig;
use tower_http::{
    cors::{Any, CorsLayer},
    set_header::SetResponseHeaderLayer,
};

use crate::axum_api::middleware::errors::MiddlewareError;

pub mod errors;

const HSTS_HEADER_NAME: &str = "strict-transport-security";
const HSTS_HEADER_VALUE: &str = "max-age=31536000; includeSubDomains; preload";

pub fn apply_middleware(mut router: Router, config: &AppConfig) -> Result<Router, MiddlewareError> {
    let mut cors_layer = CorsLayer::new().allow_methods([Method::GET, Method::POST]);

    match config.cors_origins() {
        Some(origins) => {
            for origin in origins {
                cors_layer = cors_layer.allow_origin(
                    HeaderValue::from_str(origin)
                        .map_err(|err| MiddlewareError::InvalidOrigin(err))?,
                )
            }
        }
        None => {
            cors_layer = cors_layer.allow_origin(Any);
        }
    }
    router = router.layer(cors_layer);

    if config.use_hsts() {
        let hsts_layer = SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static(HSTS_HEADER_NAME),
            HeaderValue::from_static(HSTS_HEADER_VALUE),
        );
        router = router.layer(hsts_layer);
    }

    Ok(router)
}
