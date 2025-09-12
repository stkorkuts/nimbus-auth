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
use nimbus_auth_shared::futures::PinnedFuture;
use ulid::Ulid;

pub struct PostgreSQLUserRepository {}

pub struct PostgreSQLUserRepositoryTransaction {}

impl TransactionLike for PostgreSQLUserRepositoryTransaction {
    fn commit(&mut self) -> PinnedFuture<(), TransactionError> {
        todo!()
    }

    fn rollback(&mut self) -> PinnedFuture<(), TransactionError> {
        todo!()
    }
}

impl PostgreSQLUserRepository {}

impl Transactional for PostgreSQLUserRepository {
    type TransactionType = Transaction;

    fn start_transaction(
        &self,
        isolation_level: TransactionIsolationLevel,
        block_target: TransactonBlockTarget,
    ) -> PinnedFuture<Self::TransactionType, TransactionError> {
        todo!()
    }
}

impl UserRepository for PostgreSQLUserRepository {
    fn get_by_id(
        &self,
        id: Identifier<Ulid, User>,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
    }

    fn get_by_name(
        &self,
        name: &UserName,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
    }

    fn get_by_session(
        &self,
        session: &Session<Active>,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
    }

    fn save(
        &self,
        user: &User,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<(), UserRepositoryError> {
        todo!()
    }
}
