use thiserror::Error;
use tokio::io;

#[derive(Debug, Error)]
pub enum WebApiError {
    #[error("invalid listener addr")]
    InvalidListenerAddr(#[source] io::Error),
    #[error("serve failed")]
    ServeFailed(#[source] io::Error),
}
