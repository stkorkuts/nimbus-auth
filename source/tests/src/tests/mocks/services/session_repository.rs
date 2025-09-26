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

use crate::tests::mocks::datastore::MockDatastore;

pub struct MockSessionRepository {
    datastore: Arc<MockDatastore>,
}

struct SessionSave {
    old: Option<SomeSession>,
    new: SomeSession,
}

/// Represents mock session repository with active transaction
///
/// Transaction implemented with `ReadUncomitted` isolation level which is sufficient for tests for now
pub struct MockSessionRepositoryWithTransaction {
    datastore: Arc<MockDatastore>,
    session_saves: Arc<Mutex<Vec<SessionSave>>>,
}

impl MockSessionRepository {
    pub fn new(datastore: Arc<MockDatastore>) -> Self {
        MockSessionRepository { datastore }
    }
}

impl SessionRepository for MockSessionRepository {
    fn start_transaction(
        &self,
    ) -> StaticPinnedFuture<Box<dyn SessionRepositoryWithTransaction>, SessionRepositoryError> {
        let datastore_clone: Arc<MockDatastore> = self.datastore.clone();
        pin_static_future(async move {
            Ok(Box::new(MockSessionRepositoryWithTransaction {
                datastore: datastore_clone,
                session_saves: Arc::new(Mutex::new(Vec::new())),
            }) as Box<dyn SessionRepositoryWithTransaction>)
        })
    }

    fn get_by_id(
        &self,
        id: &Identifier<Ulid, SomeSession>,
    ) -> StaticPinnedFuture<Option<SomeSession>, SessionRepositoryError> {
        let datastore_clone: Arc<MockDatastore> = self.datastore.clone();
        let id_clone = id.clone();
        pin_static_future(async move {
            Ok(datastore_clone
                .sessions()
                .get(&id_clone)
                .map(|session_ref| session_ref.value().clone()))
        })
    }

    fn save(&self, session: SomeSessionRef) -> StaticPinnedFuture<(), SessionRepositoryError> {
        let datastore_clone: Arc<MockDatastore> = self.datastore.clone();
        let session_clone = session.deref_clone();
        pin_static_future(async move {
            datastore_clone
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
            let sessions = self.datastore.sessions();
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
        id: &Identifier<Ulid, SomeSession>,
    ) -> StaticPinnedFuture<
        (
            Box<dyn SessionRepositoryWithTransaction>,
            Option<SomeSession>,
        ),
        SessionRepositoryError,
    > {
        let id_clone = id.clone();
        pin_static_future(async move {
            let session = self
                .datastore
                .sessions()
                .get(&id_clone)
                .map(|session_ref| session_ref.value().clone());
            Ok((self as Box<dyn SessionRepositoryWithTransaction>, session))
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
                .datastore
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
