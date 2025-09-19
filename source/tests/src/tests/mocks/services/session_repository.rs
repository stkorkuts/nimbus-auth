use std::sync::Arc;

use nimbus_auth_application::services::session_repository::{
    SessionRepository, SessionRepositoryWithTransaction, errors::SessionRepositoryError,
};
use nimbus_auth_domain::{
    entities::{
        Entity,
        session::{SomeSession, SomeSessionRef},
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::{StaticPinnedFuture, pin_static_future};
use tokio::sync::Mutex;
use ulid::Ulid;

use crate::tests::mocks::database::MockDatabase;

pub struct MockSessionRepository {
    database: Arc<MockDatabase>,
}

struct SessionSave {
    old: Option<SomeSession>,
    new: SomeSession,
}

/// Represents mock session repository with active transaction
///
/// Transaction implemented with `ReadUncomitted` isolation level which is sufficient for tests for now
pub struct MockSessionRepositoryWithTransaction {
    database: Arc<MockDatabase>,
    session_saves: Arc<Mutex<Vec<SessionSave>>>,
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
        let database_clone: Arc<MockDatabase> = self.database.clone();
        pin_static_future(async move {
            Ok(Box::new(MockSessionRepositoryWithTransaction {
                database: database_clone,
                session_saves: Arc::new(Mutex::new(Vec::new())),
            }) as Box<dyn SessionRepositoryWithTransaction>)
        })
    }

    fn get_by_id(
        &self,
        id: Identifier<Ulid, SomeSession>,
    ) -> StaticPinnedFuture<Option<SomeSession>, SessionRepositoryError> {
        let database_clone: Arc<MockDatabase> = self.database.clone();
        pin_static_future(async move {
            Ok(database_clone
                .sessions()
                .get(&id)
                .map(|session_ref| session_ref.value().clone()))
        })
    }

    fn save(&self, session: SomeSessionRef) -> StaticPinnedFuture<(), SessionRepositoryError> {
        let database_clone: Arc<MockDatabase> = self.database.clone();
        let session_clone = session.deref_clone();
        pin_static_future(async move {
            database_clone
                .sessions()
                .insert(session_clone.id().clone(), session_clone);
            Ok(())
        })
    }
}

impl SessionRepositoryWithTransaction for MockSessionRepositoryWithTransaction {
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), SessionRepositoryError> {
        pin_static_future(async { Ok(()) })
    }

    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), SessionRepositoryError> {
        pin_static_future(async move {
            let mut saves = self.session_saves.lock().await;
            let sessions = self.database.sessions();
            while let Some(save) = saves.pop() {
                match save.old {
                    Some(old) => {
                        sessions.insert(old.id().clone(), old.clone());
                    }
                    None => {
                        sessions.remove(save.new.id());
                    }
                }
            }
            Ok(())
        })
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
        pin_static_future(async move {
            let user = self
                .database
                .sessions()
                .get(&id)
                .map(|session_ref| session_ref.value().clone());
            Ok((self as Box<dyn SessionRepositoryWithTransaction>, user))
        })
    }

    fn save(
        self: Box<Self>,
        session: SomeSessionRef,
    ) -> StaticPinnedFuture<(Box<dyn SessionRepositoryWithTransaction>, ()), SessionRepositoryError>
    {
        let session_clone = session.deref_clone();
        pin_static_future(async move {
            let old = self
                .database
                .sessions()
                .insert(session_clone.id().clone(), session_clone.clone());

            let save_record = SessionSave {
                old,
                new: session_clone,
            };

            {
                let mut saves = self.session_saves.lock().await;
                saves.push(save_record);
            }

            Ok((self as Box<dyn SessionRepositoryWithTransaction>, ()))
        })
    }
}
