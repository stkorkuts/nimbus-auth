use std::sync::Arc;

use nimbus_auth_application::services::user_repository::{
    UserRepository, UserRepositoryWithTransaction, errors::UserRepositoryError,
};
use nimbus_auth_domain::{
    entities::{
        session::{Active, Session},
        user::{User, value_objects::name::UserName},
    },
    value_objects::identifier::{Identifier, IdentifierOfType},
};
use nimbus_auth_shared::{
    errors::ErrorBoxed,
    futures::{PinnedFuture, pin, pin_error_boxed},
};
use sqlx::{Acquire, Postgres, query, query_as};
use tokio::{spawn, sync::mpsc};
use ulid::Ulid;

use crate::{
    postgres_db::PostgresDatabase,
    services_implementations::postgres_user_repository::{
        queries::get_user_by_id, schema::UserDb,
        transactional::PostgresUserRepositoryWithTransaction,
    },
};

mod queries;
mod schema;
mod transactional;

pub struct PostgresUserRepository {
    database: Arc<PostgresDatabase>,
}

impl PostgresUserRepository {
    pub fn new(database: Arc<PostgresDatabase>) -> Self {
        Self { database }
    }
}

impl UserRepository for PostgresUserRepository {
    fn start_transaction(
        &self,
    ) -> PinnedFuture<Box<dyn UserRepositoryWithTransaction>, ErrorBoxed> {
        let db_cloned = self.database.clone();
        pin_error_boxed(async move {
            let transactional_repo = PostgresUserRepositoryWithTransaction::init(db_cloned).await?;
            Ok(Box::new(transactional_repo) as Box<dyn UserRepositoryWithTransaction>)
        })
    }

    fn get_by_id(
        &self,
        id: Identifier<Ulid, User>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError> {
        let db_clone = self.database.clone();
        pin(async move {
            let mut connection = db_clone.pool().acquire().await.map_err(ErrorBoxed::from)?;
            Ok(get_user_by_id(&mut *connection, &id.to_string())
                .await?
                .map(|user_db| user_db.into_domain())
                .transpose()?)
        })
    }

    fn get_by_name(&self, user_name: &UserName) -> PinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
    }

    fn get_by_session(
        &self,
        session: &Session<Active>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
    }

    fn save(&self, user: &User) -> PinnedFuture<(), UserRepositoryError> {
        todo!()
    }
}
