use std::sync::Arc;

use nimbus_auth_application::services::user_repository::{
    UserRepository, UserRepositoryWithTransaction, errors::UserRepositoryError,
};
use nimbus_auth_domain::{
    entities::{
        Entity,
        session::{Active, Session},
        user::{User, value_objects::name::UserName},
    },
    value_objects::identifier::{Identifier, IdentifierOfType},
};
use nimbus_auth_shared::{
    errors::ErrorBoxed,
    futures::{StaticPinnedFuture, pin_future, pin_future_error_boxed},
};
use sqlx::{Acquire, PgConnection};
use tokio::sync::oneshot;
use ulid::Ulid;

use crate::{
    postgres_db::{PostgresDatabase, PostgresTransaction},
    services_implementations::postgres_user_repository::{
        queries::{get_user_by_id, get_user_by_name, save_user},
        schema::UserDb,
    },
};

mod queries;
mod schema;

pub struct PostgresUserRepository {
    database: Arc<PostgresDatabase>,
}

enum UserRepositoryTransactionQueryRequest {
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
    transaction: PostgresTransaction<UserRepositoryTransactionQueryRequest>,
}

impl PostgresUserRepository {
    pub fn new(database: Arc<PostgresDatabase>) -> Self {
        Self { database }
    }
}

impl UserRepository for PostgresUserRepository {
    fn start_transaction(
        &self,
    ) -> StaticPinnedFuture<Box<dyn UserRepositoryWithTransaction>, ErrorBoxed> {
        let db_cloned = self.database.clone();
        pin_future_error_boxed(async move {
            let transactional_repo = PostgresUserRepositoryWithTransaction::init(db_cloned).await?;
            Ok(Box::new(transactional_repo) as Box<dyn UserRepositoryWithTransaction>)
        })
    }

    fn get_by_id(
        &self,
        id: Identifier<Ulid, User>,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError> {
        let db_clone = self.database.clone();
        let id = id.to_string();
        pin_future(async move {
            let mut connection = db_clone.pool().acquire().await.map_err(ErrorBoxed::from)?;
            get_user_by_id(&mut *connection, &id)
                .await?
                .map(|user_db| user_db.into_domain())
                .transpose()
        })
    }

    fn get_by_name(
        &self,
        user_name: &UserName,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError> {
        let db_clone = self.database.clone();
        let user_name = user_name.to_string();
        pin_future(async move {
            let mut connection = db_clone.pool().acquire().await.map_err(ErrorBoxed::from)?;
            get_user_by_name(&mut *connection, &user_name)
                .await?
                .map(|user_db| user_db.into_domain())
                .transpose()
        })
    }

    fn get_by_session(
        &self,
        session: &Session<Active>,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError> {
        let db_clone = self.database.clone();
        let session_id = session.id().to_string();
        pin_future(async move {
            let mut connection = db_clone.pool().acquire().await.map_err(ErrorBoxed::from)?;
            get_user_by_id(&mut *connection, &session_id)
                .await?
                .map(|user_db| user_db.into_domain())
                .transpose()
        })
    }

    fn save(&self, user: &User) -> StaticPinnedFuture<(), UserRepositoryError> {
        let db_clone = self.database.clone();
        let user = UserDb::from(user);
        pin_future(async move {
            let mut connection = db_clone.pool().acquire().await.map_err(ErrorBoxed::from)?;
            save_user(&mut *connection, &user).await
        })
    }
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
        request: UserRepositoryTransactionQueryRequest,
    ) -> Result<(), ErrorBoxed> {
        match request {
            UserRepositoryTransactionQueryRequest::GetById {
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
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), ErrorBoxed> {
        pin_future_error_boxed(async move { self.transaction.commit().await })
    }

    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), ErrorBoxed> {
        pin_future_error_boxed(async move { self.transaction.rollback().await })
    }

    fn get_by_id(
        self: Box<Self>,
        id: Identifier<Ulid, User>,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    > {
        let id = id.to_string();
        pin_future(async move {
            let result = self
                .transaction
                .execute::<Option<UserDb>>(Box::new(|tx| {
                    UserRepositoryTransactionQueryRequest::GetById {
                        id,
                        get_by_id_result_sender: tx,
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

    fn get_by_name(
        self: Box<Self>,
        user_name: &UserName,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    > {
        let name = user_name.to_string();
        pin_future(async move {
            let result = self
                .transaction
                .execute::<Option<UserDb>>(Box::new(|tx| {
                    UserRepositoryTransactionQueryRequest::GetByName {
                        user_name: name,
                        get_by_name_result_sender: tx,
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

    fn get_by_session(
        self: Box<Self>,
        session: &Session<Active>,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    > {
        let sid = session.id().to_string();
        pin_future(async move {
            let result = self
                .transaction
                .execute::<Option<UserDb>>(Box::new(|tx| {
                    UserRepositoryTransactionQueryRequest::GetBySession {
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
    ) -> StaticPinnedFuture<(Box<dyn UserRepositoryWithTransaction>, ()), UserRepositoryError> {
        let user_db = UserDb::from(user);
        pin_future(async move {
            let result = self
                .transaction
                .execute::<()>(Box::new(|tx| UserRepositoryTransactionQueryRequest::Save {
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
