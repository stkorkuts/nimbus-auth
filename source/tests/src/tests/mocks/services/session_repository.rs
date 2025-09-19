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

pub struct MockSessionRepository {
    sessions: Arc<DashMap<Identifier<Ulid, SomeSession>, SomeSession>>,
}

pub struct MockSessionRepositoryWithTransaction {
    sessions: Arc<DashMap<Identifier<Ulid, SomeSession>, SomeSession>>,
}

impl MockSessionRepository {
    pub fn new(sessions: Option<Vec<SomeSession>>) -> Self {
        let sessions = Arc::new(
            sessions
                .unwrap_or_default()
                .into_iter()
                .map(|session| (session.id().clone(), session))
                .collect(),
        );
        MockSessionRepository { sessions }
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
