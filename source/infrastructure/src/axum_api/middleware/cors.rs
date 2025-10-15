use axum::{
    Router,
    http::{HeaderValue, Method},
};
use tower_http::cors::{Any, CorsLayer};

use crate::axum_api::middleware::cors::errors::CorsMiddlewareError;

pub mod errors;

pub fn apply_cors_middleware(
    router: Router,
    origins: &Vec<String>,
) -> Result<Router, CorsMiddlewareError> {
    let mut cors_layer = CorsLayer::new().allow_methods([Method::GET, Method::POST]);
    match origins.len() > 0 {
        true => {
            for origin in origins {
                cors_layer = cors_layer
                    .allow_origin(HeaderValue::from_str(origin).map_err(CorsMiddlewareError::from)?)
            }
        }
        false => {
            cors_layer = cors_layer.allow_origin(Any);
        }
    }
    Ok(router.layer(cors_layer))
}
