use thiserror::Error;

use crate::services::transactions::errors::TransactionError;

#[derive(Debug, Error)]
pub enum RotateKeyPairsError {
    #[error(transparent)]
    TransactionError(#[from] TransactionError),
}
