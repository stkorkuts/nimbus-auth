use nimbus_auth_application::services::{
    transactions::{Transaction, TransactionLike, Transactional},
    user_repository::UserRepository,
};
use nimbus_auth_domain::entities::user::User;
use nimbus_auth_shared::{errors::ErrorBoxed, futures::PinnedFuture};

pub struct PostgreSQLUserRepository {}

pub struct PostgreSQLUserRepositoryTransaction {}

impl TransactionLike for PostgreSQLUserRepositoryTransaction {
    fn commit(&mut self) -> PinnedFuture<(), ErrorBoxed> {
        todo!()
    }

    fn rollback(&mut self) -> PinnedFuture<(), ErrorBoxed> {
        todo!()
    }
}

impl Transactional for PostgreSQLUserRepository {
    type TransactionType = Transaction;

    fn start_transaction(&self) -> PinnedFuture<Self::TransactionType, ErrorBoxed> {
        todo!()
    }
}

impl UserRepository for PostgreSQLUserRepository {
    fn get_by_id(
        &self,
        id: &ulid::Ulid,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<User>, ErrorBoxed> {
        todo!()
    }

    fn get_by_username(
        &self,
        username: &str,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<User>, ErrorBoxed> {
        todo!()
    }

    fn get_by_session(
        &self,
        refresh_token: &nimbus_auth_domain::entities::session::Session<
            nimbus_auth_domain::entities::session::Active,
        >,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<User>, ErrorBoxed> {
        todo!()
    }

    fn save(
        &self,
        user: &User,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<(), ErrorBoxed> {
        todo!()
    }
}
