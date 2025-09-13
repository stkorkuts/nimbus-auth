use nimbus_auth_domain::entities::user::value_objects::{
    name::errors::UserNameError, password::errors::PasswordError,
};
use thiserror::Error;

use crate::services::{
    keypair_repository::errors::KeyPairRepositoryError, time_service::errors::TimeServiceError,
    user_repository::errors::UserRepositoryError,
};

#[derive(Debug, Error)]
pub enum SignUpError {
    #[error(transparent)]
    InvalidUserName(#[from] UserNameError),
    #[error(transparent)]
    UserRepository(#[from] UserRepositoryError),
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
}
