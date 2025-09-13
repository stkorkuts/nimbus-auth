use std::sync::Arc;

use nimbus_auth_application::services::user_repository::{
    TransactionalUserRepository, UserRepository, UserRepositoryBase, errors::UserRepositoryError,
};
use nimbus_auth_domain::{
    entities::{
        session::{Active, Session},
        user::{User, value_objects::name::UserName},
    },
    value_objects::identifier::{Identifier, IdentifierOfType},
};
use nimbus_auth_shared::{
    constants::DEFAULT_CHANNEL_BUFFER_SIZE,
    errors::ErrorBoxed,
    futures::{PinnedFuture, pin, pin_error_boxed},
};
use sqlx::{Acquire, Postgres, query, query_as};
use tokio::{spawn, sync::mpsc};
use ulid::Ulid;

use crate::{
    postgres_db::PostgresDatabase,
    services_implementations::postgres_user_repository::{
        schema::UserDb, transactional::TransactionalPostgresUserRepository,
    },
};

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
    fn start_transaction(&self) -> PinnedFuture<Box<dyn TransactionalUserRepository>, ErrorBoxed> {
        let db_cloned = self.database.clone();
        pin_error_boxed(async move {
            let transactional_repo = TransactionalPostgresUserRepository::init(db_cloned).await?;
            Ok(Box::new(transactional_repo) as Box<dyn TransactionalUserRepository>)
        })
    }
}

impl UserRepositoryBase for PostgresUserRepository {
    fn get_by_id(
        &self,
        id: Identifier<Ulid, User>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
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
