use std::{error::Error, sync::Arc};

use nimbus_auth_shared::{
    errors::ErrorBoxed,
    futures::{PinnedFuture, pin, pin_error_boxed},
};
use tokio::sync::Mutex;

pub trait Transactional {
    type TransactionType: TransactionLike;
    fn start_transaction(&self) -> PinnedFuture<Self::TransactionType, ErrorBoxed>;
}

pub trait TransactionLike: Send + Sync {
    fn commit(&mut self) -> PinnedFuture<(), ErrorBoxed>;
    fn rollback(&mut self) -> PinnedFuture<(), ErrorBoxed>;
}

#[derive(Clone)]
pub struct Transaction(Arc<Mutex<Box<dyn TransactionLike>>>);

impl TransactionLike for Transaction {
    fn commit(&mut self) -> PinnedFuture<(), ErrorBoxed> {
        let inner = self.0.clone();
        pin_error_boxed(async move { inner.lock().await.commit().await })
    }

    fn rollback(&mut self) -> PinnedFuture<(), ErrorBoxed> {
        let inner = self.0.clone();
        pin_error_boxed(async move { inner.lock().await.rollback().await })
    }
}

impl Transaction {
    pub fn new(transaction: Box<dyn TransactionLike>) -> Self {
        Self(Arc::new(Mutex::new(transaction)))
    }

    pub async fn run<
        T: Send + 'static,
        F: FnOnce(Transaction) -> Fut,
        Fut: Future<Output = Result<T, ErrorBoxed>> + Send + 'static,
    >(
        &mut self,
        f: F,
    ) -> Result<T, ErrorBoxed> {
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
