use nimbus_auth_domain::{
    entities::{
        session::{Active, Session},
        user::{User, value_objects::user_name::UserName},
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::StaticPinnedFuture;
use ulid::Ulid;

use crate::services::user_repository::errors::UserRepositoryError;

pub mod errors;

pub trait UserRepository: Send + Sync {
    fn start_transaction(
        &self,
    ) -> StaticPinnedFuture<Box<dyn UserRepositoryWithTransaction>, UserRepositoryError>;
    fn get_by_id(
        &self,
        id: &Identifier<Ulid, User>,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError>;
    fn get_by_name(
        &self,
        user_name: &UserName,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError>;
    fn get_by_session(
        &self,
        session: &Session<Active>,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError>;
    fn save(&self, user: &User) -> StaticPinnedFuture<(), UserRepositoryError>;
}

pub trait UserRepositoryWithTransaction: Send + Sync {
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), UserRepositoryError>;
    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), UserRepositoryError>;
    fn get_by_id(
        self: Box<Self>,
        id: &Identifier<Ulid, User>,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    >;
    fn get_by_name(
        self: Box<Self>,
        user_name: &UserName,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    >;
    fn get_by_session(
        self: Box<Self>,
        session: &Session<Active>,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    >;
    fn save(
        self: Box<Self>,
        user: &User,
    ) -> StaticPinnedFuture<(Box<dyn UserRepositoryWithTransaction>, ()), UserRepositoryError>;
}
