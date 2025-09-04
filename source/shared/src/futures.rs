use std::{error::Error, pin::Pin};

pub type PinnedFuture<T> =
    Pin<Box<dyn Future<Output = Result<T, Box<dyn Error>>> + Send + Sync + 'static>>;

pub fn pinned<T>(
    fut: impl Future<Output = Result<T, Box<dyn Error>>> + Send + Sync + 'static,
) -> PinnedFuture<T> {
    Box::pin(fut)
}
