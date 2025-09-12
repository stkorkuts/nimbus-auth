use nimbus_auth_shared::errors::ErrorBoxed;
use thiserror::Error;

use crate::services::{
    keypair_repository::errors::KeyPairRepositoryError,
    session_repository::errors::SessionRepositoryError, time_service::errors::TimeServiceError,
    user_repository::errors::UserRepositoryError,
};

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error(transparent)]
    KeyPairRepository(#[from] KeyPairRepositoryError),
    #[error(transparent)]
    SessionRepository(#[from] SessionRepositoryError),
    #[error(transparent)]
    TimeService(#[from] TimeServiceError),
    #[error(transparent)]
    UserRepository(#[from] UserRepositoryError),
    #[error(transparent)]
    Other(#[from] ErrorBoxed),
}
