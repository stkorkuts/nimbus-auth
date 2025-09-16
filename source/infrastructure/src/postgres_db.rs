use std::{pin::Pin, sync::Arc, time::Duration};

use nimbus_auth_shared::{
    config::AppConfig,
    constants::DEFAULT_CHANNEL_BUFFER_SIZE,
    errors::ErrorBoxed,
    futures::{PinnedFuture, pin, pin_error_boxed},
};
use sqlx::{Acquire, PgConnection, PgPool, Postgres, Transaction, postgres::PgPoolOptions};
use tokio::{
    spawn,
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

use crate::postgres_db::{
    errors::PostgresDatabaseError,
    transactions::{PostgresTransaction, PostgresTransactionRequest},
};

pub mod errors;
pub mod transactions;

pub struct PostgresDatabase {
    pool: PgPool,
}

impl PostgresDatabase {
    pub async fn new(config: &AppConfig) -> Result<Self, PostgresDatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(config.postgres_db_max_connections().0)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(config.postgres_db_url())
            .await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn start_transaction<TRequest: Send + Sync + 'static, F>(
        self: Arc<Self>,
        on_query_callback: F,
    ) -> Result<PostgresTransaction<TRequest>, ErrorBoxed>
    where
        F: for<'a> Fn(
                &'a mut PgConnection,
                TRequest,
            )
                -> Pin<Box<dyn Future<Output = Result<(), ErrorBoxed>> + Send + 'a>>
            + Send
            + 'static,
    {
        let (tx_start_sender, tx_start_receiver) = oneshot::channel();
        let (tx_execute_sender, mut tx_execute_receiver) =
            mpsc::channel::<PostgresTransactionRequest<TRequest>>(DEFAULT_CHANNEL_BUFFER_SIZE);

        let transaction_task_handle = spawn(async move {
            let mut connection = match self.pool().acquire().await.map_err(ErrorBoxed::from) {
                Ok(connection) => connection,
                Err(err) => {
                    return tx_start_sender.send(Err(err)).map_err(|_| {
                        ErrorBoxed::from_str("can not send a message via tx_start_sender")
                    });
                }
            };
            let mut transaction = match connection.begin().await.map_err(ErrorBoxed::from) {
                Ok(transaction) => transaction,
                Err(err) => {
                    return tx_start_sender.send(Err(err)).map_err(|_| {
                        ErrorBoxed::from_str("can not send a message via tx_start_sender")
                    });
                }
            };
            tx_start_sender
                .send(Ok(()))
                .map_err(|_| ErrorBoxed::from_str("can not send a message via tx_start_sender"))?;

            // do transaction rollback on error
            while let Some(request) = tx_execute_receiver.recv().await {
                match request {
                    PostgresTransactionRequest::Commit { result_sender } => {
                        let commit_result = transaction.commit().await.map_err(ErrorBoxed::from);
                        return Ok(result_sender
                            .send(commit_result.map_err(ErrorBoxed::from))
                            .map_err(|_| {
                                ErrorBoxed::from_str("can not send commit result back via sender")
                            })?);
                    }
                    PostgresTransactionRequest::Rollback { result_sender } => {
                        let rollback_result =
                            transaction.rollback().await.map_err(ErrorBoxed::from);
                        return Ok(result_sender
                            .send(rollback_result.map_err(ErrorBoxed::from))
                            .map_err(|_| {
                                ErrorBoxed::from_str("can not send rollback result back via sender")
                            })?);
                    }
                    PostgresTransactionRequest::Query(request) => {
                        on_query_callback(&mut *transaction, request).await?
                    }
                    _ => todo!(),
                }
            }

            // if we get here it means channel is closed before transaction is commited
            // so we are doing rollback now
            transaction.rollback().await.map_err(ErrorBoxed::from)?;
            Ok(())
        });

        tx_start_receiver.await.map_err(ErrorBoxed::from)??;

        Ok(PostgresTransaction::new(
            transaction_task_handle,
            tx_execute_sender,
        ))
    }
}
