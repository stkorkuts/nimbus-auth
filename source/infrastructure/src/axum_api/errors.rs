use thiserror::Error;
use tokio::{io, sync::oneshot::error::RecvError};

#[derive(Debug, Error)]
pub enum WebApiError {
    #[error("invalid listener addr")]
    InvalidListenerAddr(#[source] io::Error),
    #[error("serve failed")]
    ServeFailed(#[source] io::Error),
    #[error("can not get shutdown signal")]
    ShutdownSignal(#[from] RecvError),
}
