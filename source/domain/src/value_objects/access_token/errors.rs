use jsonwebtoken::errors::Error;
use thiserror::Error;
use ulid::DecodeError;

#[derive(Debug, Error)]
pub enum SignAccessTokenError {
    #[error("invalid private key format, should be pem. Error: {0}")]
    InvalidPrivateKeyFormat(#[source] Error),
    #[error("encoding access token Error: {0}")]
    Encoding(#[source] Error),
}

#[derive(Debug, Error)]
pub enum ExtractKeyIdError {
    #[error("header decoding error: {0}")]
    HeaderDecoding(#[source] Error),
    #[error("key id is missing from header")]
    KeyIdIsMissing,
    #[error("wrong key id format. Error: {0}")]
    WrongKeyIdFormat(#[source] DecodeError),
}

#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("invalid decoding key. Error: {0}")]
    InvalidDecodingKey(#[source] Error),
    #[error("decoding error: {0}")]
    Decoding(#[source] Error),
    #[error("invalid claims. Error: {0}")]
    InvalidClaims(String),
}
