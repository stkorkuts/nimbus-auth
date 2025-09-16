use std::{ops::Deref, sync::Arc};

use nimbus_auth_shared::{errors::ErrorBoxed, futures::PinnedFuture};
use tokio::{
    sync::{mpsc::Sender, oneshot},
    task::JoinHandle,
};

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
