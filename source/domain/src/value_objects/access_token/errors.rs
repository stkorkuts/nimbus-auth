use jsonwebtoken::errors::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignAccessTokenError {
    #[error("invalid private key format, should be pem")]
    InvalidPrivateKeyFormat(#[source] Error),
    #[error("encoding access token error")]
    EncodingError(#[source] Error),
}
