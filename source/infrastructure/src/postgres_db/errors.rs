use nimbus_auth_shared::errors::ErrorBoxed;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostgresDatabaseError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Other(#[from] ErrorBoxed),
}
