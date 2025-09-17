use nimbus_auth_shared::errors::ErrorBoxed;
use thiserror::Error;

use crate::services::keypair_repository::errors::KeyPairRepositoryError;

#[derive(Debug, Error)]
pub enum GetPublicKeyError {
    #[error(transparent)]
    KeyPairRepository(#[from] KeyPairRepositoryError),
    #[error("active key is not found")]
    KeyPairNotFound,
    #[error("key pair is revoked")]
    KeyPairIsRevoked,
    #[error("key pair is expired")]
    KeyPairIsExpired,
    #[error(transparent)]
    Other(#[from] ErrorBoxed),
}
