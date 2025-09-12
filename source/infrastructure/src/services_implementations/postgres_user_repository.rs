use std::sync::Arc;

use nimbus_auth_application::services::{
    transactions::{
        Transaction, TransactionIsolationLevel, TransactionLike, Transactional,
        TransactonBlockTarget, errors::TransactionError,
    },
    user_repository::{UserRepository, errors::UserRepositoryError},
};
use nimbus_auth_domain::{
    entities::{
        session::{Active, Session},
        user::{User, value_objects::name::UserName},
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::{
    errors::ErrorBoxed,
    futures::{PinnedFuture, pin},
};
use sqlx::Acquire;
use ulid::Ulid;

use crate::postgres_db::PostgresDatabase;

pub struct PostgresUserRepository {
    database: Arc<PostgresDatabase>,
}

pub struct PostgresUserRepositoryTransaction {
    inner_transaction: Arc<sqlx::Transaction<'static, sqlx::Postgres>>,
}

impl TransactionLike for PostgresUserRepositoryTransaction {
    fn commit(&mut self) -> PinnedFuture<(), TransactionError> {
        todo!()
    }

    fn rollback(&mut self) -> PinnedFuture<(), TransactionError> {
        todo!()
    }
}

impl PostgresUserRepository {
    pub fn new(database: Arc<PostgresDatabase>) -> Self {
        Self { database }
    }
}

impl PostgresUserRepositoryTransaction {
    pub fn new(transaction: Arc<sqlx::Transaction<'static, sqlx::Postgres>>) -> Self {
        Self {
            inner_transaction: transaction,
        }
    }
}

impl Transactional for PostgresUserRepository {
    fn start_transaction(
        &self,
        isolation_level: TransactionIsolationLevel,
        block_target: TransactonBlockTarget,
    ) -> PinnedFuture<Transaction, TransactionError> {
        let db = self.database.clone();
        pin(async move {
            let mut conn = db
                .pool()
                .acquire()
                .await
                .map_err(|err| ErrorBoxed::from(err))?;
            let transaction = Arc::new(conn.begin().await.map_err(|err| ErrorBoxed::from(err))?);
            Ok(Transaction::new(Box::new(
                PostgresUserRepositoryTransaction::new(transaction),
            )))
        })
    }
}

impl UserRepository for PostgresUserRepository {
    fn get_by_id(
        &self,
        id: Identifier<Ulid, User>,
        transaction: Option<Transaction>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
    }

    fn get_by_name(
        &self,
        name: &UserName,
        transaction: Option<Transaction>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
    }

    fn get_by_session(
        &self,
        session: &Session<Active>,
        transaction: Option<Transaction>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
    }

    fn save(
        &self,
        user: &User,
        transaction: Option<Transaction>,
    ) -> PinnedFuture<(), UserRepositoryError> {
        todo!()
    }
}
