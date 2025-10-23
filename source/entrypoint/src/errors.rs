use nimbus_auth_infrastructure::web_api::errors::WebApiError;
use nimbus_auth_shared::errors::ErrorBoxed;
use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
pub enum EntryPointError {
    #[error("error sending shutdown signal")]
    ShutdownSignalSending,
    #[error(transparent)]
    WebApi(#[from] WebApiError),
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Other(#[from] ErrorBoxed),
}
