use axum::http::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CorsMiddlewareError {
    #[error(transparent)]
    InvalidOriginHeaderValue(#[from] InvalidHeaderValue),
}
