use nimbus_auth_domain::{
    entities::{
        session::{Active, Session},
        user::{User, value_objects::name::UserName},
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::{errors::ErrorBoxed, futures::PinnedFuture};
use ulid::Ulid;

use crate::services::user_repository::errors::UserRepositoryError;

pub mod errors;

pub trait UserRepository: Send + Sync {
    fn start_transaction(&self)
    -> PinnedFuture<Box<dyn UserRepositoryWithTransaction>, ErrorBoxed>;
    fn get_by_id(
        &self,
        id: Identifier<Ulid, User>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError>;
    fn get_by_name(&self, user_name: &UserName) -> PinnedFuture<Option<User>, UserRepositoryError>;
    fn get_by_session(
        &self,
        session: &Session<Active>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError>;
    fn save(&self, user: &User) -> PinnedFuture<(), UserRepositoryError>;
}

pub trait UserRepositoryWithTransaction: Send + Sync {
    fn commit(self: Box<Self>) -> PinnedFuture<(), ErrorBoxed>;
    fn rollback(self: Box<Self>) -> PinnedFuture<(), ErrorBoxed>;
    fn get_by_id(
        self: Box<Self>,
        id: Identifier<Ulid, User>,
    ) -> PinnedFuture<(Box<dyn UserRepositoryWithTransaction>, Option<User>), UserRepositoryError>;
    fn get_by_name(
        self: Box<Self>,
        user_name: &UserName,
    ) -> PinnedFuture<(Box<dyn UserRepositoryWithTransaction>, Option<User>), UserRepositoryError>;
    fn get_by_session(
        self: Box<Self>,
        session: &Session<Active>,
    ) -> PinnedFuture<(Box<dyn UserRepositoryWithTransaction>, Option<User>), UserRepositoryError>;
    fn save(
        self: Box<Self>,
        user: &User,
    ) -> PinnedFuture<(Box<dyn UserRepositoryWithTransaction>, ()), UserRepositoryError>;
}
