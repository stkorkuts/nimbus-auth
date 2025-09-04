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

impl Transaction {
    pub fn new(transaction: Box<dyn TransactionLike>) -> Self {
        Self(Arc::new(Mutex::new(transaction)))
    }
}

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

pub trait TransactionExt {
    fn run<T: Send + Sync + 'static>(
        &mut self,
        f: Box<dyn FnOnce(Transaction) -> PinnedFuture<T, ErrorBoxed> + Send + Sync>,
    ) -> PinnedFuture<T, ErrorBoxed>;
}

impl TransactionExt for Transaction {
    fn run<T: Send + Sync + 'static>(
        &mut self,
        f: Box<dyn FnOnce(Self) -> PinnedFuture<T, ErrorBoxed> + Send + Sync>,
    ) -> PinnedFuture<T, ErrorBoxed> {
        let mut transaction = self.clone();
        pin_error_boxed(async move {
            let result = f(transaction.clone()).await;
            match result {
                Ok(value) => {
                    transaction.commit().await?;
                    Ok(value)
                }
                Err(err) => {
                    transaction.rollback().await?;
                    Err(err)
                }
            }
        })
    }
}
