use nimbus_auth_domain::{
    entities::session::{InitializedSession, InitializedSessionRef, Session, Uninitialized},
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::PinnedFuture;
use ulid::Ulid;

use crate::services::{
    session_repository::errors::SessionRepositoryError,
    transactions::{Transaction, Transactional},
};

pub mod errors;

pub trait SessionRepository: Transactional<TransactionType = Transaction> + Send + Sync {
    fn get_by_id(
        &self,
        id: Identifier<Ulid, Session<Uninitialized>>,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<InitializedSession>, SessionRepositoryError>;
    fn save(
        &self,
        session: InitializedSessionRef,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<(), SessionRepositoryError>;
}
