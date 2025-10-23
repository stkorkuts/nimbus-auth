use nimbus_auth_domain::entities::user::value_objects::user_name::errors::UserNameError;
use thiserror::Error;
use ulid::DecodeError;

#[derive(Error, Debug)]
pub enum SessionDbIntoDomainError {
    #[error("invalid identifier. Error: {0}")]
    InvalidIdentifier(#[from] DecodeError),
    #[error(transparent)]
    InvalidUserName(#[from] UserNameError),
    #[error("invalid user role value: {0}")]
    InvalidUserRole(String),
}
