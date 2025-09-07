use nimbus_auth_application::services::{
    transactions::{Transaction, TransactionLike, Transactional},
    user_repository::UserRepository,
};
use nimbus_auth_domain::{
    entities::user::{User, value_objects::UserName},
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::{errors::ErrorBoxed, futures::PinnedFuture};

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

impl Transactional for PostgreSQLUserRepository {
    type TransactionType = Transaction;

    fn start_transaction(&self) -> PinnedFuture<Self::TransactionType, TransactionError> {
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
        name: UserName,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
    }

    fn get_by_session(
        &self,
        refresh_token: &Session<Active>,
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
