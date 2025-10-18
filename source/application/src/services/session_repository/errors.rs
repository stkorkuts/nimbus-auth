use nimbus_auth_shared::errors::ErrorBoxed;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionRepositoryError {
    #[error("can not restore session. Error: {0}")]
    SessionRestoration(#[source] ErrorBoxed),
    #[error(transparent)]
    Other(#[from] ErrorBoxed),
}
