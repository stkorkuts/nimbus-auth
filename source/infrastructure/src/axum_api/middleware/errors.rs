use axum::http::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MiddlewareError {
    #[error("invalid origin value: {0}")]
    InvalidOrigin(#[source] InvalidHeaderValue),
}
