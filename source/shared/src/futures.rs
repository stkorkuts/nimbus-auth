use std::{error::Error, pin::Pin};

use crate::errors::ErrorBoxed;

pub type PinnedFuture<'a, T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'a>>;

pub type StaticPinnedFuture<T, E> = PinnedFuture<'static, T, E>;

pub fn pin_future<'a, T, E: Error + Send + Sync>(
    fut: impl Future<Output = Result<T, E>> + Send + 'a,
) -> PinnedFuture<'a, T, E> {
    Box::pin(fut)
}

pub fn pin_future_error_boxed<'a, T>(
    fut: impl Future<Output = Result<T, ErrorBoxed>> + Send + 'a,
) -> PinnedFuture<'a, T, ErrorBoxed> {
    Box::pin(fut)
}

pub fn pin_static_future<T, E: Error + Send + Sync>(
    fut: impl Future<Output = Result<T, E>> + Send + 'static,
) -> StaticPinnedFuture<T, E> {
    Box::pin(fut)
}

pub fn pin_static_future_error_boxed<T>(
    fut: impl Future<Output = Result<T, ErrorBoxed>> + Send + 'static,
) -> StaticPinnedFuture<T, ErrorBoxed> {
    Box::pin(fut)
}
