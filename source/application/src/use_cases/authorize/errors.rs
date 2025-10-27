use nimbus_auth_domain::value_objects::access_token::errors::{
    ExtractKeyIdError, VerificationError,
};
use thiserror::Error;

use crate::services::keypair_repository::errors::KeyPairRepositoryError;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error(transparent)]
    ExtractKeyId(#[from] ExtractKeyIdError),
    #[error(transparent)]
    KeyPairRepository(#[from] KeyPairRepositoryError),
    #[error("keypair that has been used to sign a token was not found")]
    KeyPairNotFound,
    #[error("keypair that has been used to sign a token expired")]
    KeyPairExpired,
    #[error("keypair that has been used to sign a token revoked")]
    KeyPairRevoked,
    #[error(transparent)]
    AccessTokenVerification(#[from] VerificationError),
}
