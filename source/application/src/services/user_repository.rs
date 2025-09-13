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

pub trait UserRepositoryBase: Send + Sync {
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

pub trait UserRepository: UserRepositoryBase {
    fn start_transaction(&self) -> PinnedFuture<Box<dyn TransactionalUserRepository>, ErrorBoxed>;
}

pub trait TransactionalUserRepository: UserRepositoryBase {
    fn commit(self) -> PinnedFuture<(), ErrorBoxed>;
    fn rollback(self) -> PinnedFuture<(), ErrorBoxed>;
}
