use nimbus_auth_domain::{
    entities::user::value_objects::{name::errors::UserNameError, password::errors::PasswordError},
    value_objects::access_token::errors::SignAccessTokenError,
};
use nimbus_auth_shared::errors::ErrorBoxed;
use thiserror::Error;

use crate::services::{
    keypair_repository::errors::KeyPairRepositoryError,
    session_repository::errors::SessionRepositoryError, time_service::errors::TimeServiceError,
    user_repository::errors::UserRepositoryError,
};

#[derive(Debug, Error)]
pub enum SignUpError {
    #[error(transparent)]
    InvalidUserName(#[from] UserNameError),
    #[error(transparent)]
    UserRepository(#[from] UserRepositoryError),
    #[error(transparent)]
    SessionRepository(#[from] SessionRepositoryError),
    #[error("user with name: {user_name} already exists")]
    UserAlreadyExists { user_name: String },
    #[error(transparent)]
    InvalidPassword(#[from] PasswordError),
    #[error(transparent)]
    TimeService(#[from] TimeServiceError),
    #[error(transparent)]
    KeyPairRepository(#[from] KeyPairRepositoryError),
    #[error("active key pair not found")]
    ActiveKeyPairNotFound,
    #[error(transparent)]
    SignAccessToken(#[from] SignAccessTokenError),
    #[error(transparent)]
    Other(#[from] ErrorBoxed),
}
