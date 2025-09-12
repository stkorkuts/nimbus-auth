use nimbus_auth_domain::{
    entities::{
        session::{Active, Session},
        user::{User, value_objects::name::UserName},
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::PinnedFuture;
use ulid::Ulid;

use crate::services::{
    transactions::{Transaction, Transactional},
    user_repository::errors::UserRepositoryError,
};

pub mod errors;

pub trait UserRepository: Transactional + Send + Sync {
    fn get_by_id(
        &self,
        id: Identifier<Ulid, User>,
        transaction: Option<Transaction>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError>;
    fn get_by_name(
        &self,
        user_name: &UserName,
        transaction: Option<Transaction>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError>;
    fn get_by_session(
        &self,
        session: &Session<Active>,
        transaction: Option<Transaction>,
    ) -> PinnedFuture<Option<User>, UserRepositoryError>;
    fn save(
        &self,
        user: &User,
        transaction: Option<Transaction>,
    ) -> PinnedFuture<(), UserRepositoryError>;
}
