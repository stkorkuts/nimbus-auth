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
use sqlx::{Acquire, PgConnection};
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
    postgres_db::{PostgresDatabase, transactions::PostgresTransaction},
    services_implementations::postgres_user_repository::{queries::get_user_by_id, schema::UserDb},
};

enum UserRepositoryQueryRequest {
    GetById {
        id: String,
        get_by_id_result_sender: oneshot::Sender<Result<Option<UserDb>, ErrorBoxed>>,
    },
    GetByName {
        user_name: String,
        get_by_name_result_sender: oneshot::Sender<Result<Option<UserDb>, ErrorBoxed>>,
    },
    GetBySession {
        session_id: String,
        get_by_session_result_sender: oneshot::Sender<Result<Option<UserDb>, ErrorBoxed>>,
    },
    Save {
        user: UserDb,
        save_result_sender: oneshot::Sender<Result<(), ErrorBoxed>>,
    },
}

pub struct PostgresUserRepositoryWithTransaction {
    transaction: PostgresTransaction<UserRepositoryQueryRequest>,
}

impl PostgresUserRepositoryWithTransaction {
    pub async fn init(database: Arc<PostgresDatabase>) -> Result<Self, ErrorBoxed> {
        let transaction = database
            .start_transaction(|conn, req| Box::pin(Self::handle_request(conn, req)))
            .await?;
        Ok(Self { transaction })
    }

    async fn handle_request(
        connection: &mut PgConnection,
        request: UserRepositoryQueryRequest,
    ) -> Result<(), ErrorBoxed> {
        match request {
            UserRepositoryQueryRequest::GetById {
                id,
                get_by_id_result_sender,
            } => {
                let result = get_user_by_id(connection, &id)
                    .await
                    .map_err(ErrorBoxed::from);
                get_by_id_result_sender
                    .send(result)
                    .map_err(|_| ErrorBoxed::from_str("can not send result back via sender"))?;
                Ok(())
            }
            _ => todo!(),
        }
    }
}

impl UserRepositoryWithTransaction for PostgresUserRepositoryWithTransaction {
    fn commit(self: Box<Self>) -> PinnedFuture<(), ErrorBoxed> {
        pin_error_boxed(async move { self.transaction.commit().await })
    }

    fn rollback(self: Box<Self>) -> PinnedFuture<(), ErrorBoxed> {
        pin_error_boxed(async move { self.transaction.rollback().await })
    }

    fn get_by_id(
        self: Box<Self>,
        id: Identifier<Ulid, User>,
    ) -> PinnedFuture<(Box<dyn UserRepositoryWithTransaction>, Option<User>), UserRepositoryError>
    {
        let id = id.to_string();
        pin(async move {
            let result = self
                .transaction
                .execute::<Option<UserDb>>(Box::new(|tx| UserRepositoryQueryRequest::GetById {
                    id,
                    get_by_id_result_sender: tx,
                }))
                .await
                .map_err(UserRepositoryError::from)?;

            Ok((
                Box::new(Self {
                    transaction: result.0,
                }) as Box<dyn UserRepositoryWithTransaction>,
                result
                    .1
                    .map(|db| db.into_domain())
                    .transpose()
                    .map_err(ErrorBoxed::from)?,
            ))
        })
    }

    fn get_by_name(
        self: Box<Self>,
        user_name: &UserName,
    ) -> PinnedFuture<(Box<dyn UserRepositoryWithTransaction>, Option<User>), UserRepositoryError>
    {
        let name = user_name.to_string();
        pin(async move {
            let result = self
                .transaction
                .execute::<Option<UserDb>>(Box::new(|tx| UserRepositoryQueryRequest::GetByName {
                    user_name: name,
                    get_by_name_result_sender: tx,
                }))
                .await
                .map_err(UserRepositoryError::from)?;

            Ok((
                Box::new(Self {
                    transaction: result.0,
                }) as Box<dyn UserRepositoryWithTransaction>,
                result
                    .1
                    .map(|db| db.into_domain())
                    .transpose()
                    .map_err(ErrorBoxed::from)?,
            ))
        })
    }

    fn get_by_session(
        self: Box<Self>,
        session: &Session<Active>,
    ) -> PinnedFuture<(Box<dyn UserRepositoryWithTransaction>, Option<User>), UserRepositoryError>
    {
        let sid = session.id().to_string();
        pin(async move {
            let result = self
                .transaction
                .execute::<Option<UserDb>>(Box::new(|tx| {
                    UserRepositoryQueryRequest::GetBySession {
                        session_id: sid,
                        get_by_session_result_sender: tx,
                    }
                }))
                .await
                .map_err(UserRepositoryError::from)?;

            Ok((
                Box::new(Self {
                    transaction: result.0,
                }) as Box<dyn UserRepositoryWithTransaction>,
                result
                    .1
                    .map(|db| db.into_domain())
                    .transpose()
                    .map_err(ErrorBoxed::from)?,
            ))
        })
    }

    fn save(
        self: Box<Self>,
        user: &User,
    ) -> PinnedFuture<(Box<dyn UserRepositoryWithTransaction>, ()), UserRepositoryError> {
        let user_db = UserDb::from(user);
        pin(async move {
            let result = self
                .transaction
                .execute::<()>(Box::new(|tx| UserRepositoryQueryRequest::Save {
                    user: user_db,
                    save_result_sender: tx,
                }))
                .await
                .map_err(UserRepositoryError::from)?;

            Ok((
                Box::new(Self {
                    transaction: result.0,
                }) as Box<dyn UserRepositoryWithTransaction>,
                result.1,
            ))
        })
    }
}
