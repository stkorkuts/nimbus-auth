use std::pin::Pin;

pub type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

pub fn pinned<T>(fut: impl Future<Output = T> + Send + 'static) -> PinnedFuture<T> {
    Box::pin(fut)
}
