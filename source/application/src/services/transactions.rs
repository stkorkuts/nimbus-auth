use std::sync::Arc;

use nimbus_auth_shared::futures::{PinnedFuture, pin};
use tokio::sync::Mutex;

use crate::services::transactions::errors::TransactionError;

pub mod errors;

pub enum TransactionIsolationLevel {
    Default,
    ReadCommited,
    RepeatableRead,
    Serializable,
}

pub enum TransactonBlockTarget {
    Default,
    Row,
    Table,
}

pub trait Transactional {
    type TransactionType: TransactionLike;
    fn start_transaction(
        &self,
        isolation_level: TransactionIsolationLevel,
        block_target: TransactonBlockTarget,
    ) -> PinnedFuture<Self::TransactionType, TransactionError>;
}

pub trait TransactionLike: Send + Sync {
    fn commit(&mut self) -> PinnedFuture<(), TransactionError>;
    fn rollback(&mut self) -> PinnedFuture<(), TransactionError>;
}

#[derive(Clone)]
pub struct Transaction(Arc<Mutex<Box<dyn TransactionLike>>>);

impl TransactionLike for Transaction {
    fn commit(&mut self) -> PinnedFuture<(), TransactionError> {
        let inner = self.0.clone();
        pin(async move { inner.lock().await.commit().await })
    }

    fn rollback(&mut self) -> PinnedFuture<(), TransactionError> {
        let inner = self.0.clone();
        pin(async move { inner.lock().await.rollback().await })
    }
}

impl Transaction {
    pub fn new(transaction: Box<dyn TransactionLike>) -> Self {
        Self(Arc::new(Mutex::new(transaction)))
    }

    pub async fn run<
        T: Send + 'static,
        F: FnOnce(Transaction) -> Fut,
        Fut: Future<Output = Result<T, TransactionError>> + Send + 'static,
    >(
        &mut self,
        f: F,
    ) -> Result<T, TransactionError> {
        let mut tx = self.clone();
        let result = f(tx.clone()).await;
        match result {
            Ok(val) => {
                tx.commit().await?;
                Ok(val)
            }
            Err(err) => {
                tx.rollback().await?;
                Err(err)
            }
        }
    }
}
