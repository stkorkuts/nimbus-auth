use nimbus_auth_shared::errors::ErrorBoxed;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionRepositoryError {
    #[error(transparent)]
    Other(#[from] ErrorBoxed),
}
