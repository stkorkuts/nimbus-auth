use std::error::Error;

use nimbus_auth_shared::errors::ErrorBoxed;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RotateKeyPairsError {
    #[error("keypairs repository error")]
    KeyPairsRepositoryError(#[source] ErrorBoxed),
    #[error("transaction error")]
    TransactionError(#[source] ErrorBoxed),
    #[error("time service error")]
    TimeServiceError(#[source] ErrorBoxed),
}
