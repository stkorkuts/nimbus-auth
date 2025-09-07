use thiserror::Error;

use crate::services::keypair_repository::errors::KeyPairRepositoryError;

#[derive(Debug, Error)]
pub enum GetPublicKeyError {
    #[error(transparent)]
    KeyPairRepository(#[from] KeyPairRepositoryError),
    #[error("active key is not found")]
    ActiveKeyPairNotFound,
}
