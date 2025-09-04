use std::{error::Error, sync::Arc};

use nimbus_auth_shared::futures::{PinnedFuture, pinned};
use tokio::sync::Mutex;

pub trait Transactional {
    type TransactionType: Transaction;
    fn start_transaction(&self) -> PinnedFuture<Self::TransactionType>;
}

pub trait Transaction: Send + Sync {
    fn commit(&mut self) -> PinnedFuture<()>;
    fn rollback(&mut self) -> PinnedFuture<()>;
}

#[derive(Clone)]
pub struct TransactionWrapper(Arc<Mutex<dyn Transaction>>);

impl TransactionWrapper {
    pub fn new(transaction: Arc<Mutex<dyn Transaction>>) -> Self {
        Self(transaction)
    }
}

impl Transaction for TransactionWrapper {
    fn commit(&mut self) -> PinnedFuture<()> {
        let inner = self.0.clone();
        pinned(async move { inner.lock().await.commit().await })
    }

    fn rollback(&mut self) -> PinnedFuture<()> {
        let inner = self.0.clone();
        pinned(async move { inner.lock().await.rollback().await })
    }
}
