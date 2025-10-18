use nimbus_auth_shared::errors::ErrorBoxed;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("can not restore user from db. Error: {0}")]
    UserRestoration(#[source] ErrorBoxed),
    #[error("session is not found")]
    SessionIsNotFound,
    #[error(transparent)]
    Other(#[from] ErrorBoxed),
}
