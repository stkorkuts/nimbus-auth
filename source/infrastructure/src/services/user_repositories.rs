use std::sync::Arc;

use nimbus_auth_application::services::{
    transactions::{Transaction, TransactionLike, Transactional},
    user_repository::UserRepository,
};
use nimbus_auth_shared::futures::pinned;

pub struct PostgreSQLUserRepository {}

pub struct PostgreSQLUserRepositoryTransaction {}

impl TransactionLike for PostgreSQLUserRepositoryTransaction {
    fn commit(&mut self) -> nimbus_auth_shared::futures::PinnedFuture<()> {
        todo!()
    }

    fn rollback(&mut self) -> nimbus_auth_shared::futures::PinnedFuture<()> {
        todo!()
    }
}

impl Transactional for PostgreSQLUserRepository {
    type TransactionType = Transaction;

    fn start_transaction(
        &self,
    ) -> nimbus_auth_shared::futures::PinnedFuture<Self::TransactionType> {
        pinned(async {
            Ok(Transaction::new(Box::new(
                PostgreSQLUserRepositoryTransaction {},
            )))
        })
    }
}

impl UserRepository for PostgreSQLUserRepository {
    fn get_by_id(
        &self,
        id: &ulid::Ulid,
        transaction: Option<Self::TransactionType>,
    ) -> nimbus_auth_shared::futures::PinnedFuture<Option<nimbus_auth_domain::entities::user::User>>
    {
        todo!()
    }

    fn get_by_username(
        &self,
        username: &str,
        transaction: Option<Self::TransactionType>,
    ) -> nimbus_auth_shared::futures::PinnedFuture<Option<nimbus_auth_domain::entities::user::User>>
    {
        todo!()
    }

    fn get_by_session(
        &self,
        refresh_token: &nimbus_auth_domain::entities::session::Session<
            nimbus_auth_domain::entities::session::Active,
        >,
        transaction: Option<Self::TransactionType>,
    ) -> nimbus_auth_shared::futures::PinnedFuture<Option<nimbus_auth_domain::entities::user::User>>
    {
        todo!()
    }

    fn save(
        &self,
        user: &nimbus_auth_domain::entities::user::User,
        transaction: Option<Self::TransactionType>,
    ) -> nimbus_auth_shared::futures::PinnedFuture<()> {
        todo!()
    }
}
