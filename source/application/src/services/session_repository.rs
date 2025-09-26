use nimbus_auth_domain::{
    entities::session::{SomeSession, SomeSessionRef},
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
        id: &Identifier<Ulid, SomeSession>,
    ) -> StaticPinnedFuture<Option<SomeSession>, SessionRepositoryError>;
    fn save(&self, session: SomeSessionRef) -> StaticPinnedFuture<(), SessionRepositoryError>;
}

pub trait SessionRepositoryWithTransaction: Send + Sync {
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), SessionRepositoryError>;
    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), SessionRepositoryError>;
    fn get_by_id(
        self: Box<Self>,
        id: &Identifier<Ulid, SomeSession>,
    ) -> StaticPinnedFuture<
        (
            Box<dyn SessionRepositoryWithTransaction>,
            Option<SomeSession>,
        ),
        SessionRepositoryError,
    >;
    fn save(
        self: Box<Self>,
        session: SomeSessionRef,
    ) -> StaticPinnedFuture<(Box<dyn SessionRepositoryWithTransaction>, ()), SessionRepositoryError>;
}
