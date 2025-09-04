use std::{error::Error, pin::Pin};

use crate::errors::ErrorBoxed;

pub type PinnedFuture<T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send + Sync + 'static>>;

pub fn pin<T, E: Error + Send + Sync>(
    fut: impl Future<Output = Result<T, E>> + Send + Sync + 'static,
) -> PinnedFuture<T, E> {
    Box::pin(fut)
}

pub fn pin_error_boxed<T>(
    fut: impl Future<Output = Result<T, ErrorBoxed>> + Send + Sync + 'static,
) -> PinnedFuture<T, ErrorBoxed> {
    Box::pin(fut)
}

pub trait PinnedFutureExt<T, E> {
    fn box_error(self) -> PinnedFuture<T, ErrorBoxed>;
}

impl<T: 'static, E: Error + Send + Sync + 'static> PinnedFutureExt<T, E> for PinnedFuture<T, E> {
    fn box_error(self) -> PinnedFuture<T, ErrorBoxed> {
        pin_error_boxed(async move {
            match self.await {
                Ok(val) => Ok(val),
                Err(err) => Err(Box::new(err) as ErrorBoxed),
            }
        })
    }
}
