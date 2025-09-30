use nimbus_auth_domain::entities::user::value_objects::{
    user_name::errors::UserNameError, password_hash::errors::PasswordHashError,
};
use nimbus_auth_shared::errors::ErrorBoxed;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error(transparent)]
    UserName(#[from] UserNameError),
    #[error(transparent)]
    PasswordHash(#[from] PasswordHashError),
    #[error("session is not found")]
    SessionIsNotFound,
    #[error(transparent)]
    Other(#[from] ErrorBoxed),
}
