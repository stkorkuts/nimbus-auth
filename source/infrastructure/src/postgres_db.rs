use std::{pin::Pin, sync::Arc, time::Duration};

use nimbus_auth_shared::{
    config::AppConfig, constants::DEFAULT_CHANNEL_BUFFER_SIZE, errors::ErrorBoxed,
    futures::PinnedFuture,
};
use sqlx::{Acquire, PgConnection, PgPool, postgres::PgPoolOptions};
use tokio::{
    spawn,
    sync::{
        mpsc::{self, Sender},
        oneshot,
    },
    task::JoinHandle,
};

use crate::postgres_db::errors::PostgresDatabaseError;

pub mod errors;

pub struct PostgresDatabase {
    pool: PgPool,
}

#[must_use = "Transaction must be committed or rolled back before being dropped"]
pub struct PostgresTransaction<TRequest> {
    transaction_handle: JoinHandle<Result<(), ErrorBoxed>>,
    execute_sender: Sender<PostgresTransactionRequest<TRequest>>,
}

pub enum PostgresTransactionRequest<TRequest> {
    Query(TRequest),
    Commit {
        result_sender: oneshot::Sender<Result<(), ErrorBoxed>>,
    },
    Rollback {
        result_sender: oneshot::Sender<Result<(), ErrorBoxed>>,
    },
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

    pub async fn start_transaction<
        TRequest: Send + Sync + 'static,
        F: for<'a> Fn(&'a mut PgConnection, TRequest) -> PinnedFuture<'a, (), ErrorBoxed>
            + Send
            + 'static,
    >(
        self: Arc<Self>,
        on_query_callback: F,
    ) -> Result<PostgresTransaction<TRequest>, ErrorBoxed> {
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

            while let Some(request) = tx_execute_receiver.recv().await {
                match request {
                    PostgresTransactionRequest::Commit { result_sender } => {
                        let commit_result = transaction.commit().await.map_err(ErrorBoxed::from);
                        return result_sender.send(commit_result).map_err(|_| {
                            ErrorBoxed::from_str("can not send commit result back via sender")
                        });
                    }
                    PostgresTransactionRequest::Rollback { result_sender } => {
                        let rollback_result =
                            transaction.rollback().await.map_err(ErrorBoxed::from);
                        return result_sender.send(rollback_result).map_err(|_| {
                            ErrorBoxed::from_str("can not send rollback result back via sender")
                        });
                    }
                    PostgresTransactionRequest::Query(request) => {
                        on_query_callback(&mut transaction, request).await?
                    }
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

impl<TRequest: Send + Sync + 'static> PostgresTransaction<TRequest> {
    pub fn new(
        transaction_handle: JoinHandle<Result<(), ErrorBoxed>>,
        execute_sender: Sender<PostgresTransactionRequest<TRequest>>,
    ) -> Self {
        Self {
            transaction_handle,
            execute_sender,
        }
    }

    pub async fn rollback(self) -> Result<(), ErrorBoxed> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.execute_sender
            .send(PostgresTransactionRequest::Rollback { result_sender })
            .await
            .map_err(ErrorBoxed::from)?;
        result_receiver.await.map_err(ErrorBoxed::from)??;
        self.transaction_handle.await?
    }

    pub async fn commit(self) -> Result<(), ErrorBoxed> {
        let (result_sender, result_receiver) = oneshot::channel();
        self.execute_sender
            .send(PostgresTransactionRequest::Commit { result_sender })
            .await
            .map_err(ErrorBoxed::from)?;
        result_receiver.await.map_err(ErrorBoxed::from)??;
        self.transaction_handle.await?
    }

    pub async fn execute<TResponse>(
        self,
        build_request: Box<
            dyn FnOnce(oneshot::Sender<Result<TResponse, ErrorBoxed>>) -> TRequest
                + Send
                + Sync
                + 'static,
        >,
    ) -> Result<(Self, TResponse), ErrorBoxed> {
        let (result_sender, result_receiver) = oneshot::channel();

        let request = build_request(result_sender);

        if let Err(err) = self
            .execute_sender
            .send(PostgresTransactionRequest::Query(request))
            .await
        {
            self.rollback().await?;
            return Err(ErrorBoxed::from(err));
        }

        match result_receiver.await {
            Ok(res) => res,
            Err(_) => {
                self.rollback().await?;
                return Err(ErrorBoxed::from_str(
                    "can not get result back via result receiver",
                ));
            }
        }
        .map(|val| (self, val))
    }
}
