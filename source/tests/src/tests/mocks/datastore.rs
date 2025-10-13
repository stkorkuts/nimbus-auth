use std::sync::Arc;

use dashmap::DashMap;
use nimbus_auth_domain::{
    entities::{Entity, keypair::SomeKeyPair, session::SomeSession, user::User},
    value_objects::identifier::Identifier,
};
use ulid::Ulid;

pub struct MockDatastore {
    users: Arc<DashMap<Identifier<Ulid, User>, User>>,
    sessions: Arc<DashMap<Identifier<Ulid, SomeSession<'static>>, SomeSession<'static>>>,
    keypairs: Arc<DashMap<Identifier<Ulid, SomeKeyPair<'static>>, SomeKeyPair<'static>>>,
}

impl MockDatastore {
    pub fn new(
        users: Option<Vec<User>>,
        sessions: Option<Vec<SomeSession<'static>>>,
        keypairs: Option<Vec<SomeKeyPair<'static>>>,
    ) -> Self {
        Self {
            users: Arc::new(
                users
                    .unwrap_or_default()
                    .into_iter()
                    .map(|user| (user.id().clone(), user))
                    .collect(),
            ),
            sessions: Arc::new(
                sessions
                    .unwrap_or_default()
                    .into_iter()
                    .map(|session| (session.id().clone(), session))
                    .collect(),
            ),
            keypairs: Arc::new(
                keypairs
                    .unwrap_or_default()
                    .into_iter()
                    .map(|keypair| (keypair.id().clone(), keypair))
                    .collect(),
            ),
        }
    }

    pub fn users(&self) -> Arc<DashMap<Identifier<Ulid, User>, User>> {
        self.users.clone()
    }

    pub fn sessions(
        &self,
    ) -> Arc<DashMap<Identifier<Ulid, SomeSession<'static>>, SomeSession<'static>>> {
        self.sessions.clone()
    }

    pub fn keypairs(
        &self,
    ) -> Arc<DashMap<Identifier<Ulid, SomeKeyPair<'static>>, SomeKeyPair<'static>>> {
        self.keypairs.clone()
    }
}
