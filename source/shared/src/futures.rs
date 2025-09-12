use std::{error::Error, pin::Pin};

use crate::errors::ErrorBoxed;

pub type PinnedFuture<T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'static>>;

pub fn pin<T, E: Error + Send + Sync>(
    fut: impl Future<Output = Result<T, E>> + Send + 'static,
) -> PinnedFuture<T, E> {
    Box::pin(fut)
}

pub fn pin_error_boxed<T>(
    fut: impl Future<Output = Result<T, ErrorBoxed>> + Send + 'static,
) -> PinnedFuture<T, ErrorBoxed> {
    Box::pin(fut)
}
