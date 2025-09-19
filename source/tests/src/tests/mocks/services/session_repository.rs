use std::sync::Arc;

use dashmap::DashMap;
use nimbus_auth_application::services::session_repository::{
    SessionRepository, SessionRepositoryWithTransaction, errors::SessionRepositoryError,
};
use nimbus_auth_domain::{
    entities::{
        Entity,
        keypair::Active,
        session::{Session, SomeSession, SomeSessionRef},
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::StaticPinnedFuture;
use ulid::Ulid;

use crate::tests::mocks::database::MockDatabase;

pub struct MockSessionRepository {
    database: Arc<MockDatabase>,
}

/// Represents mock session repository with active transaction
///
/// Transaction implemented with `ReadUncomitted` isolation level which is sufficient for tests for now
pub struct MockSessionRepositoryWithTransaction {
    database: Arc<MockDatabase>,
}

impl MockSessionRepository {
    pub fn new(database: Arc<MockDatabase>) -> Self {
        MockSessionRepository { database }
    }
}

impl SessionRepository for MockSessionRepository {
    fn start_transaction(
        &self,
    ) -> StaticPinnedFuture<Box<dyn SessionRepositoryWithTransaction>, SessionRepositoryError> {
        todo!()
    }

    fn get_by_id(
        &self,
        id: Identifier<Ulid, SomeSession>,
    ) -> StaticPinnedFuture<Option<SomeSession>, SessionRepositoryError> {
        todo!()
    }

    fn save(&self, session: SomeSessionRef) -> StaticPinnedFuture<(), SessionRepositoryError> {
        todo!()
    }
}

impl SessionRepositoryWithTransaction for MockSessionRepositoryWithTransaction {
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), SessionRepositoryError> {
        todo!()
    }

    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), SessionRepositoryError> {
        todo!()
    }

    fn get_by_id(
        self: Box<Self>,
        id: Identifier<Ulid, SomeSession>,
    ) -> StaticPinnedFuture<
        (
            Box<dyn SessionRepositoryWithTransaction>,
            Option<SomeSession>,
        ),
        SessionRepositoryError,
    > {
        todo!()
    }

    fn save(
        self: Box<Self>,
        session: SomeSessionRef,
    ) -> StaticPinnedFuture<(Box<dyn SessionRepositoryWithTransaction>, ()), SessionRepositoryError>
    {
        todo!()
    }
}
