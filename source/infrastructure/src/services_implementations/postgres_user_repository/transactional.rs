use std::sync::Arc;

use nimbus_auth_application::services::user_repository::{
    TransactionalUserRepository, UserRepositoryBase, errors::UserRepositoryError,
};
use nimbus_auth_domain::{
    entities::{
        Entity,
        session::{Active, Session},
        user::{
            User,
            specifications::RestoreUserSpecification,
            value_objects::{name::UserName, password_hash::PasswordHash},
        },
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::{
    constants::DEFAULT_CHANNEL_BUFFER_SIZE,
    errors::ErrorBoxed,
    futures::{PinnedFuture, pin, pin_error_boxed},
};
use sqlx::Acquire;
use tokio::{
    spawn,
    sync::{
        Mutex,
        mpsc::{self, Receiver, Sender},
        oneshot,
    },
    task::JoinHandle,
};
use ulid::Ulid;

use crate::{
    postgres_db::PostgresDatabase,
    services_implementations::postgres_user_repository::schema::UserDb,
};

enum QueryRequest {
    Commit {
        commit_result_sender: oneshot::Sender<Result<(), UserRepositoryError>>,
    },
    Rollback {
        rollback_result_sender: oneshot::Sender<Result<(), UserRepositoryError>>,
    },
    GetById {
        id: String,
        get_by_id_result_sender: oneshot::Sender<Result<Option<UserDb>, UserRepositoryError>>,
    },
    GetByName {
        user_name: String,
        get_by_name_result_sender: oneshot::Sender<Result<Option<UserDb>, UserRepositoryError>>,
    },
    GetBySession {
        session_id: String,
        get_by_name_result_sender: oneshot::Sender<Result<Option<UserDb>, UserRepositoryError>>,
    },
    Save {
        user: UserDb,
        save_result_sender: oneshot::Sender<Result<(), UserRepositoryError>>,
    },
}

pub struct TransactionalPostgresUserRepository {
    transaction_execute_sender: Sender<QueryRequest>,
    transaction_task_handle: JoinHandle<Result<(), ErrorBoxed>>,
}

impl TransactionalPostgresUserRepository {
    pub fn init(database: Arc<PostgresDatabase>) -> PinnedFuture<Self, ErrorBoxed> {
        let (tx_start_sender, tx_start_receiver) = oneshot::channel();
        let (tx_execute_sender, mut tx_execute_receiver) =
            mpsc::channel::<QueryRequest>(DEFAULT_CHANNEL_BUFFER_SIZE);

        let transaction_task_handle = spawn(async move {
            let mut connection = match database.pool().acquire().await.map_err(ErrorBoxed::from) {
                Ok(connection) => connection,
                Err(err) => {
                    return tx_start_sender.send(Err(err)).map_err(|_| {
                        ErrorBoxed::from_str("can not send a message via tx_start_sender")
                    });
                }
            };
            let transaction = match connection.begin().await.map_err(ErrorBoxed::from) {
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

            while let Some(query_request) = tx_execute_receiver.recv().await {
                match query_request {
                    QueryRequest::Commit {
                        commit_result_sender,
                    } => {
                        let commit_result = transaction.commit().await.map_err(ErrorBoxed::from);
                        return Ok(commit_result_sender
                            .send(commit_result.map_err(UserRepositoryError::from))
                            .map_err(|_| {
                                ErrorBoxed::from_str(
                                    "can not send commit result back to transaction",
                                )
                            })?);
                    }
                    QueryRequest::Rollback {
                        rollback_result_sender,
                    } => {
                        let rollback_result =
                            transaction.rollback().await.map_err(ErrorBoxed::from);
                        return Ok(rollback_result_sender
                            .send(rollback_result.map_err(UserRepositoryError::from))
                            .map_err(|_| {
                                ErrorBoxed::from_str(
                                    "can not send rollback result back to transaction",
                                )
                            })?);
                    }
                    _ => todo!(),
                }
            }

            // if we get here it means channel is closed before transaction is commited
            // so we are doing rollback now
            transaction.rollback().await.map_err(ErrorBoxed::from)?;
            Ok(())
        });

        pin_error_boxed(async move {
            tx_start_receiver.await.map_err(ErrorBoxed::from)??;

            Ok(TransactionalPostgresUserRepository {
                transaction_task_handle,
                transaction_execute_sender: tx_execute_sender,
            })
        })
    }

    fn send_to_transaction<T: Send + 'static>(
        query_request: QueryRequest,
        transaction_execute_sender: Sender<QueryRequest>,
        result_receiver: oneshot::Receiver<Result<T, UserRepositoryError>>,
    ) -> PinnedFuture<T, UserRepositoryError> {
        pin(async move {
            transaction_execute_sender
                .send(query_request)
                .await
                .map_err(ErrorBoxed::from)?;

            let result = result_receiver.await.map_err(|_| {
                ErrorBoxed::from_str("can not send result back to result receiver")
            })??;

            Ok(result)
        })
    }
}

impl TransactionalUserRepository for TransactionalPostgresUserRepository {
    fn commit(self) -> PinnedFuture<(), ErrorBoxed> {
        let (result_sender, result_receiver) = oneshot::channel();
        let query_request = QueryRequest::Commit {
            commit_result_sender: result_sender,
        };
        let transaction_execute_sender = self.transaction_execute_sender.clone();
        pin_error_boxed(async move {
            TransactionalPostgresUserRepository::send_to_transaction(
                query_request,
                transaction_execute_sender,
                result_receiver,
            )
            .await?;

            self.transaction_task_handle.await??;
            Ok(())
        })
    }

    fn rollback(self) -> PinnedFuture<(), ErrorBoxed> {
        let (result_sender, result_receiver) = oneshot::channel();
        let query_request = QueryRequest::Rollback {
            rollback_result_sender: result_sender,
        };
        let transaction_execute_sender = self.transaction_execute_sender.clone();
        pin_error_boxed(async move {
            TransactionalPostgresUserRepository::send_to_transaction(
                query_request,
                transaction_execute_sender,
                result_receiver,
            )
            .await?;

            self.transaction_task_handle.await??;

            Ok(())
        })
    }
}

impl UserRepositoryBase for TransactionalPostgresUserRepository {
    fn get_by_id(
        &self,
        id: Identifier<Ulid, User>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError> {
        let (result_sender, result_receiver) = oneshot::channel();
        let query_request = QueryRequest::GetById {
            id: id.to_string(),
            get_by_id_result_sender: result_sender,
        };
        let transaction_execute_sender = self.transaction_execute_sender.clone();
        pin(async move {
            Ok(TransactionalPostgresUserRepository::send_to_transaction(
                query_request,
                transaction_execute_sender,
                result_receiver,
            )
            .await?
            .map(|user_db| user_db.into_domain())
            .transpose()?)
        })
    }

    fn get_by_name(&self, user_name: &UserName) -> PinnedFuture<Option<User>, UserRepositoryError> {
        let (result_sender, result_receiver) = oneshot::channel();
        let query_request = QueryRequest::GetByName {
            user_name: user_name.to_string(),
            get_by_name_result_sender: result_sender,
        };
        let transaction_execute_sender = self.transaction_execute_sender.clone();
        pin(async move {
            Ok(TransactionalPostgresUserRepository::send_to_transaction(
                query_request,
                transaction_execute_sender,
                result_receiver,
            )
            .await?
            .map(|user_db| user_db.into_domain())
            .transpose()?)
        })
    }

    fn get_by_session(
        &self,
        session: &Session<Active>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError> {
        let (result_sender, result_receiver) = oneshot::channel();
        let query_request = QueryRequest::GetBySession {
            session_id: session.id().to_string(),
            get_by_name_result_sender: result_sender,
        };
        let transaction_execute_sender = self.transaction_execute_sender.clone();
        pin(async move {
            Ok(TransactionalPostgresUserRepository::send_to_transaction(
                query_request,
                transaction_execute_sender,
                result_receiver,
            )
            .await?
            .map(|user_db| user_db.into_domain())
            .transpose()?)
        })
    }

    fn save(&self, user: &User) -> PinnedFuture<(), UserRepositoryError> {
        let (result_sender, result_receiver) = oneshot::channel();
        let query_request = QueryRequest::Save {
            user: UserDb::from(user),
            save_result_sender: result_sender,
        };
        let transaction_execute_sender = self.transaction_execute_sender.clone();
        pin(async move {
            Ok(TransactionalPostgresUserRepository::send_to_transaction(
                query_request,
                transaction_execute_sender,
                result_receiver,
            )
            .await?)
        })
    }
}
