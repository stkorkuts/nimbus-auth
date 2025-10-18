use nimbus_auth_domain::entities::user::value_objects::{
    password_hash::errors::PasswordHashError, user_name::errors::UserNameError,
};
use thiserror::Error;
use ulid::DecodeError;

#[derive(Error, Debug)]
pub enum TryFromUserDbError {
    #[error("invalid identifier. Error: {0}")]
    InvalidIdentifier(#[from] DecodeError),
    #[error(transparent)]
    UserName(#[from] UserNameError),
    #[error(transparent)]
    PasswordHash(#[from] PasswordHashError),
}
