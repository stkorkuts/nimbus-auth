use std::sync::Arc;

use nimbus_auth_application::services::user_repository::{
    UserRepositoryWithTransaction, errors::UserRepositoryError,
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
    services_implementations::postgres_user_repository::{queries::get_user_by_id, schema::UserDb},
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

pub struct PostgresUserRepositoryWithTransaction {
    transaction_execute_sender: Sender<QueryRequest>,
    transaction_task_handle: JoinHandle<Result<(), ErrorBoxed>>,
}

impl PostgresUserRepositoryWithTransaction {
    pub async fn init(database: Arc<PostgresDatabase>) -> Result<Self, ErrorBoxed> {
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
            while let Some(query_request) = tx_execute_receiver.recv().await {
                match query_request {
                    QueryRequest::Commit {
                        commit_result_sender,
                    } => {
                        let commit_result = transaction.commit().await.map_err(ErrorBoxed::from);
                        return Ok(commit_result_sender
                            .send(commit_result.map_err(UserRepositoryError::from))
                            .map_err(|_| {
                                ErrorBoxed::from_str("can not send commit result back via sender")
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
                                ErrorBoxed::from_str("can not send rollback result back via sender")
                            })?);
                    }
                    QueryRequest::GetById {
                        id,
                        get_by_id_result_sender,
                    } => {
                        let query_result = get_user_by_id(&mut *transaction, &id).await;
                        get_by_id_result_sender.send(query_result).map_err(|_| {
                            ErrorBoxed::from_str("can not send result back via sender")
                        })?;
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

        Ok(PostgresUserRepositoryWithTransaction {
            transaction_task_handle,
            transaction_execute_sender: tx_execute_sender,
        })
    }

    async fn execute_query<
        T,
        F: FnOnce(oneshot::Sender<Result<T, UserRepositoryError>>) -> QueryRequest,
    >(
        self: Box<Self>,
        build_request: F,
    ) -> Result<(Box<dyn UserRepositoryWithTransaction>, T), UserRepositoryError> {
        let (result_sender, result_receiver) = oneshot::channel();
        let query_request = build_request(result_sender);

        if let Err(err) = self.transaction_execute_sender.send(query_request).await {
            self.rollback().await?;
            return Err(UserRepositoryError::from(ErrorBoxed::from(err)));
        }

        match result_receiver.await {
            Ok(res) => res,
            Err(_) => {
                self.rollback().await?;
                return Err(UserRepositoryError::from(ErrorBoxed::from_str(
                    "can not get result back via result receiver",
                )));
            }
        }
        .map(|val| (self as Box<dyn UserRepositoryWithTransaction>, val))
    }
}

impl UserRepositoryWithTransaction for PostgresUserRepositoryWithTransaction {
    fn commit(self: Box<Self>) -> PinnedFuture<(), ErrorBoxed> {
        let transaction_execute_sender = self.transaction_execute_sender.clone();
        pin_error_boxed(async move {
            let (result_sender, result_receiver) = oneshot::channel();
            let query_request = QueryRequest::Commit {
                commit_result_sender: result_sender,
            };
            transaction_execute_sender
                .send(query_request)
                .await
                .map_err(ErrorBoxed::from)?;

            let result = result_receiver.await.map_err(|_| {
                ErrorBoxed::from_str("can not get result back via result receiver")
            })??;

            self.transaction_task_handle.await??;
            Ok(result)
        })
    }

    fn rollback(self: Box<Self>) -> PinnedFuture<(), ErrorBoxed> {
        let transaction_execute_sender = self.transaction_execute_sender.clone();
        pin_error_boxed(async move {
            let (result_sender, result_receiver) = oneshot::channel();
            let query_request = QueryRequest::Rollback {
                rollback_result_sender: result_sender,
            };
            transaction_execute_sender
                .send(query_request)
                .await
                .map_err(ErrorBoxed::from)?;

            let result = result_receiver.await.map_err(|_| {
                ErrorBoxed::from_str("can not get result back via result receiver")
            })??;

            self.transaction_task_handle.await??;

            Ok(result)
        })
    }

    fn get_by_id(
        self: Box<Self>,
        id: Identifier<Ulid, User>,
    ) -> PinnedFuture<(Box<dyn UserRepositoryWithTransaction>, Option<User>), UserRepositoryError>
    {
        pin(async move {
            self.execute_query(|tx| QueryRequest::GetById {
                id: id.to_string(),
                get_by_id_result_sender: tx,
            })
            .await
            .map(|(repo, user)| {
                let user = user.map(|db| db.into_domain()).transpose()?;
                Ok((repo, user))
            })?
        })
    }

    fn get_by_name(
        self: Box<Self>,
        user_name: &UserName,
    ) -> PinnedFuture<(Box<dyn UserRepositoryWithTransaction>, Option<User>), UserRepositoryError>
    {
        let name = user_name.to_string();
        pin(async move {
            self.execute_query(|tx| QueryRequest::GetByName {
                user_name: name,
                get_by_name_result_sender: tx,
            })
            .await
            .map(|(repo, user)| {
                let user = user.map(|db| db.into_domain()).transpose()?;
                Ok((repo, user))
            })?
        })
    }

    fn get_by_session(
        self: Box<Self>,
        session: &Session<Active>,
    ) -> PinnedFuture<(Box<dyn UserRepositoryWithTransaction>, Option<User>), UserRepositoryError>
    {
        let sid = session.id().to_string();
        pin(async move {
            self.execute_query(|tx| QueryRequest::GetBySession {
                session_id: sid,
                get_by_name_result_sender: tx,
            })
            .await
            .map(|(repo, user)| {
                let user = user.map(|db| db.into_domain()).transpose()?;
                Ok((repo, user))
            })?
        })
    }

    fn save(
        self: Box<Self>,
        user: &User,
    ) -> PinnedFuture<(Box<dyn UserRepositoryWithTransaction>, ()), UserRepositoryError> {
        let user_db = UserDb::from(user);
        pin(async move {
            self.execute_query(|tx| QueryRequest::Save {
                user: user_db,
                save_result_sender: tx,
            })
            .await
        })
    }
}
