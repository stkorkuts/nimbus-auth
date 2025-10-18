use std::sync::Arc;

use nimbus_auth_application::services::user_repository::{
    UserRepository, UserRepositoryWithTransaction, errors::UserRepositoryError,
};
use nimbus_auth_domain::{
    entities::{
        Entity,
        session::{Active, Session},
        user::{User, value_objects::user_name::UserName},
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::{
    errors::ErrorBoxed,
    futures::{StaticPinnedFuture, pin_future, pin_static_future},
};
use sqlx::PgConnection;
use ulid::Ulid;

use crate::{
    postgres_db::{PostgresDatabase, PostgresTransaction},
    services_implementations::postgres_user_repository::{
        queries::{get_user_by_id, get_user_by_name, get_user_by_session, save_user},
        schema::{GetUserDb, SaveUserDb},
    },
};

mod queries;
mod schema;

pub struct PostgresUserRepository {
    database: Arc<PostgresDatabase>,
}

enum UserRepositoryTransactionQueryRequest {
    GetById { id: String },
    GetByName { user_name: String },
    GetBySession { session_id: String },
    Save { user: SaveUserDb },
}

enum UserRepositoryTransactionQueryResponse {
    OptionalUser { user: Option<GetUserDb> },
    UserSaved,
}

pub struct PostgresUserRepositoryWithTransaction {
    transaction: PostgresTransaction<
        UserRepositoryTransactionQueryRequest,
        UserRepositoryTransactionQueryResponse,
        UserRepositoryError,
    >,
}

impl PostgresUserRepository {
    pub fn new(database: Arc<PostgresDatabase>) -> Self {
        Self { database }
    }
}

impl UserRepository for PostgresUserRepository {
    fn start_transaction(
        &self,
    ) -> StaticPinnedFuture<Box<dyn UserRepositoryWithTransaction>, UserRepositoryError> {
        let db_cloned = self.database.clone();
        pin_static_future(async move {
            let transactional_repo = PostgresUserRepositoryWithTransaction::init(db_cloned).await?;
            Ok(Box::new(transactional_repo) as Box<dyn UserRepositoryWithTransaction>)
        })
    }

    fn get_by_id(
        &self,
        id: &Identifier<Ulid, User>,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError> {
        let db_clone = self.database.clone();
        let id = id.to_string();
        pin_static_future(async move {
            let mut connection = db_clone.pool().acquire().await.map_err(ErrorBoxed::from)?;
            get_user_by_id(&mut *connection, &id)
                .await?
                .map(|user_db| {
                    User::try_from(&user_db)
                        .map_err(|err| UserRepositoryError::UserRestoration(ErrorBoxed::from(err)))
                })
                .transpose()
        })
    }

    fn get_by_name(
        &self,
        user_name: &UserName,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError> {
        let db_clone = self.database.clone();
        let user_name = user_name.to_string();
        pin_static_future(async move {
            let mut connection = db_clone.pool().acquire().await.map_err(ErrorBoxed::from)?;
            get_user_by_name(&mut *connection, &user_name)
                .await?
                .map(|user_db| {
                    User::try_from(&user_db)
                        .map_err(|err| UserRepositoryError::UserRestoration(ErrorBoxed::from(err)))
                })
                .transpose()
        })
    }

    fn get_by_session(
        &self,
        session: &Session<Active>,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError> {
        let db_clone = self.database.clone();
        let session_id = session.id().to_string();
        pin_static_future(async move {
            let mut connection = db_clone.pool().acquire().await.map_err(ErrorBoxed::from)?;
            get_user_by_id(&mut *connection, &session_id)
                .await?
                .map(|user_db| {
                    User::try_from(&user_db)
                        .map_err(|err| UserRepositoryError::UserRestoration(ErrorBoxed::from(err)))
                })
                .transpose()
        })
    }

    fn save(&self, user: &User) -> StaticPinnedFuture<(), UserRepositoryError> {
        let db_clone = self.database.clone();
        let user = SaveUserDb::from(user);
        pin_static_future(async move {
            let mut connection = db_clone.pool().acquire().await.map_err(ErrorBoxed::from)?;
            save_user(&mut *connection, &user).await
        })
    }
}

impl PostgresUserRepositoryWithTransaction {
    pub async fn init(database: Arc<PostgresDatabase>) -> Result<Self, UserRepositoryError> {
        let transaction = database
            .start_transaction(|conn, req| pin_future(Self::handle_request(conn, req)))
            .await?;
        Ok(Self { transaction })
    }

    async fn handle_request(
        connection: &mut PgConnection,
        request: UserRepositoryTransactionQueryRequest,
    ) -> Result<UserRepositoryTransactionQueryResponse, UserRepositoryError> {
        match request {
            UserRepositoryTransactionQueryRequest::GetById { id } => {
                Ok(UserRepositoryTransactionQueryResponse::OptionalUser {
                    user: get_user_by_id(connection, &id).await?,
                })
            }
            UserRepositoryTransactionQueryRequest::GetByName { user_name } => {
                Ok(UserRepositoryTransactionQueryResponse::OptionalUser {
                    user: get_user_by_name(connection, &user_name).await?,
                })
            }
            UserRepositoryTransactionQueryRequest::GetBySession { session_id } => {
                Ok(UserRepositoryTransactionQueryResponse::OptionalUser {
                    user: get_user_by_session(connection, &session_id).await?,
                })
            }
            UserRepositoryTransactionQueryRequest::Save { user } => {
                save_user(connection, &user).await?;
                Ok(UserRepositoryTransactionQueryResponse::UserSaved)
            }
        }
    }
}

impl UserRepositoryWithTransaction for PostgresUserRepositoryWithTransaction {
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), UserRepositoryError> {
        pin_static_future(async move { self.transaction.commit().await })
    }

    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), UserRepositoryError> {
        pin_static_future(async move { self.transaction.rollback().await })
    }

    fn get_by_id(
        self: Box<Self>,
        id: &Identifier<Ulid, User>,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    > {
        let id = id.to_string();
        pin_static_future(async move {
            let result = self
                .transaction
                .execute(UserRepositoryTransactionQueryRequest::GetById { id })
                .await
                .map_err(UserRepositoryError::from)?;

            match result.1 {
                UserRepositoryTransactionQueryResponse::OptionalUser { user } => Ok((
                    Box::new(Self {
                        transaction: result.0,
                    }) as Box<dyn UserRepositoryWithTransaction>,
                    user.map(|db| {
                        User::try_from(&db).map_err(|err| {
                            UserRepositoryError::UserRestoration(ErrorBoxed::from(err))
                        })
                    })
                    .transpose()?,
                )),
                _ => Err(UserRepositoryError::from(ErrorBoxed::from_str(
                    "got invalid response for query",
                ))),
            }
        })
    }

    fn get_by_name(
        self: Box<Self>,
        user_name: &UserName,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    > {
        let user_name = user_name.to_string();
        pin_static_future(async move {
            let result = self
                .transaction
                .execute(UserRepositoryTransactionQueryRequest::GetByName { user_name })
                .await
                .map_err(UserRepositoryError::from)?;

            match result.1 {
                UserRepositoryTransactionQueryResponse::OptionalUser { user } => Ok((
                    Box::new(Self {
                        transaction: result.0,
                    }) as Box<dyn UserRepositoryWithTransaction>,
                    user.map(|db| {
                        User::try_from(&db).map_err(|err| {
                            UserRepositoryError::UserRestoration(ErrorBoxed::from(err))
                        })
                    })
                    .transpose()?,
                )),
                _ => Err(UserRepositoryError::from(ErrorBoxed::from_str(
                    "got invalid response for query",
                ))),
            }
        })
    }

    fn get_by_session(
        self: Box<Self>,
        session: &Session<Active>,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    > {
        let session_id = session.id().to_string();
        pin_static_future(async move {
            let result = self
                .transaction
                .execute(UserRepositoryTransactionQueryRequest::GetBySession { session_id })
                .await
                .map_err(UserRepositoryError::from)?;

            match result.1 {
                UserRepositoryTransactionQueryResponse::OptionalUser { user } => Ok((
                    Box::new(Self {
                        transaction: result.0,
                    }) as Box<dyn UserRepositoryWithTransaction>,
                    user.map(|db| {
                        User::try_from(&db).map_err(|err| {
                            UserRepositoryError::UserRestoration(ErrorBoxed::from(err))
                        })
                    })
                    .transpose()?,
                )),
                _ => Err(UserRepositoryError::from(ErrorBoxed::from_str(
                    "got invalid response for query",
                ))),
            }
        })
    }

    fn save(
        self: Box<Self>,
        user: &User,
    ) -> StaticPinnedFuture<(Box<dyn UserRepositoryWithTransaction>, ()), UserRepositoryError> {
        let user = SaveUserDb::from(user);
        pin_static_future(async move {
            let result = self
                .transaction
                .execute(UserRepositoryTransactionQueryRequest::Save { user })
                .await
                .map_err(UserRepositoryError::from)?;

            match result.1 {
                UserRepositoryTransactionQueryResponse::UserSaved => Ok((
                    Box::new(Self {
                        transaction: result.0,
                    }) as Box<dyn UserRepositoryWithTransaction>,
                    (),
                )),
                _ => Err(UserRepositoryError::from(ErrorBoxed::from_str(
                    "got invalid response for query",
                ))),
            }
        })
    }
}
