use thiserror::Error;

use crate::axum_api::middleware::cors::errors::CorsMiddlewareError;

#[derive(Error, Debug)]
pub enum MiddlewareError {
    #[error(transparent)]
    Cors(#[from] CorsMiddlewareError),
}
