use nimbus_auth_domain::{
    entities::session::{InitializedSession, InitializedSessionRef, Session, Uninitialized},
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::StaticPinnedFuture;
use ulid::Ulid;

use crate::services::session_repository::errors::SessionRepositoryError;

pub mod errors;

pub trait SessionRepository: Send + Sync {
    fn start_transaction(
        &self,
    ) -> StaticPinnedFuture<Box<dyn SessionRepositoryWithTransaction>, SessionRepositoryError>;
    fn get_by_id(
        &self,
        id: Identifier<Ulid, Session<Uninitialized>>,
    ) -> StaticPinnedFuture<Option<InitializedSession>, SessionRepositoryError>;
    fn save(
        &self,
        session: InitializedSessionRef,
    ) -> StaticPinnedFuture<(), SessionRepositoryError>;
}

pub trait SessionRepositoryWithTransaction: Send + Sync {
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), SessionRepositoryError>;
    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), SessionRepositoryError>;
    fn get_by_id(
        self: Box<Self>,
        id: Identifier<Ulid, Session<Uninitialized>>,
    ) -> StaticPinnedFuture<
        (
            Box<dyn SessionRepositoryWithTransaction>,
            Option<InitializedSession>,
        ),
        SessionRepositoryError,
    >;
    fn save(
        self: Box<Self>,
        session: InitializedSessionRef,
    ) -> StaticPinnedFuture<(Box<dyn SessionRepositoryWithTransaction>, ()), SessionRepositoryError>;
}
