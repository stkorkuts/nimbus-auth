use nimbus_auth_domain::entities::session::{InitializedSession, InitializedSessionRef};
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
        id: &Ulid,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<InitializedSession>, SessionRepositoryError>;
    fn save(
        &self,
        session: InitializedSessionRef,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<(), SessionRepositoryError>;
}
