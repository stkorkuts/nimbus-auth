use thiserror::Error;
use ulid::DecodeError;

use crate::services::{
    keypair_repository::errors::KeyPairRepositoryError,
    session_repository::errors::SessionRepositoryError, time_service::errors::TimeServiceError,
    user_repository::errors::UserRepositoryError,
};

#[derive(Debug, Error)]
pub enum RefreshError {
    #[error(transparent)]
    IdDecode(#[from] DecodeError),
    #[error(transparent)]
    SessionRepository(#[from] SessionRepositoryError),
    #[error("session is not found")]
    SessionIsNotFound,
    #[error("session is expired")]
    SessionIsExpired,
    #[error("session is revoked")]
    SessionIsRevoked,
    #[error(transparent)]
    UserRepository(#[from] UserRepositoryError),
    #[error("user for this session is not found")]
    UserIsNotFound,
    #[error(transparent)]
    TimeService(#[from] TimeServiceError),
    #[error(transparent)]
    KeyPairRepository(#[from] KeyPairRepositoryError),
    #[error("active key pair not found")]
    ActiveKeyPairNotFound,
}
