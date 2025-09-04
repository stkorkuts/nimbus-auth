use std::sync::Arc;

use nimbus_auth_shared::futures::{PinnedFuture, pinned};
use tokio::sync::Mutex;

pub trait Transactional {
    type TransactionType: TransactionLike;
    fn start_transaction(&self) -> PinnedFuture<Self::TransactionType>;
}

pub trait TransactionLike: Send + Sync {
    fn commit(&mut self) -> PinnedFuture<()>;
    fn rollback(&mut self) -> PinnedFuture<()>;
}

#[derive(Clone)]
pub struct Transaction(Arc<Mutex<Box<dyn TransactionLike>>>);

impl Transaction {
    pub fn new(transaction: Box<dyn TransactionLike>) -> Self {
        Self(Arc::new(Mutex::new(transaction)))
    }
}

impl TransactionLike for Transaction {
    fn commit(&mut self) -> PinnedFuture<()> {
        let inner = self.0.clone();
        pinned(async move { inner.lock().await.commit().await })
    }

    fn rollback(&mut self) -> PinnedFuture<()> {
        let inner = self.0.clone();
        pinned(async move { inner.lock().await.rollback().await })
    }
}
