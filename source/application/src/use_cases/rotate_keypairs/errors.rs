use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RotateKeyPairsError {
    #[error("keypairs repository error")]
    KeyPairsRepositoryError(#[source] Box<dyn Error>),
    #[error("transaction error")]
    TransactionError(#[source] Box<dyn Error>),
    #[error("time service error")]
    TimeServiceError(#[source] Box<dyn Error>),
}
