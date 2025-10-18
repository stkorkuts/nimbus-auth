use thiserror::Error;
use ulid::DecodeError;

#[derive(Error, Debug)]
pub enum SessionDbIntoDomainError {
    #[error("invalid identifier. Error: {0}")]
    InvalidIdentifier(#[from] DecodeError),
}
