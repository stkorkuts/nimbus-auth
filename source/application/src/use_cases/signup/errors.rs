use nimbus_auth_domain::entities::user::value_objects::{
    name::errors::UserNameError, password::errors::PasswordError,
};
use thiserror::Error;

use crate::services::{
    keypair_repository::errors::KeyPairRepositoryError, time_service::errors::TimeServiceError,
    transactions::errors::TransactionError, user_repository::errors::UserRepositoryError,
};

#[derive(Debug, Error)]
pub enum SignUpError {
    #[error(transparent)]
    UserRepository(#[from] UserRepositoryError),
    #[error(transparent)]
    InvalidUserName(#[from] UserNameError),
    #[error("user with name: {user_name} already exists")]
    UserAlreadyExists { user_name: String },
    #[error(transparent)]
    InvalidPassword(#[from] PasswordError),
    #[error(transparent)]
    KeyPairRepository(#[from] KeyPairRepositoryError),
    #[error("active key pair not found")]
    ActiveKeyPairNotFound,
    #[error(transparent)]
    TimeService(#[from] TimeServiceError),
    #[error(transparent)]
    TransactionError(#[from] TransactionError),
}
