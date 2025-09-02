pub mod errors;
pub mod specifications;

use time::UtcDateTime;
use ulid::Ulid;

use crate::{
    entities::{
        Entity,
        session::specifications::{NewSessionSpecification, RestoreSessionSpecification},
        user::User,
    },
    value_objects::identifier::{Identifier, IdentifierOfType},
};

pub trait SessionState {}

pub struct Uninitialized {}

pub struct Active {
    expires_at: UtcDateTime,
}

pub struct Expired {
    expired_at: UtcDateTime,
}

pub struct Revoked {
    revoked_at: UtcDateTime,
}

pub struct Session<State: SessionState> {
    id: Identifier<Ulid, Session<State>>,
    user_id: Identifier<Ulid, User>,
    state: State,
}

pub enum InitializedSession {
    Active(Session<Active>),
    Revoked(Session<Revoked>),
    Expired(Session<Expired>),
}

impl<State: SessionState> Entity<Ulid> for Session<State> {
    type Id = Identifier<Ulid, Session<State>>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl SessionState for Uninitialized {}
impl SessionState for Active {}
impl SessionState for Expired {}
impl SessionState for Revoked {}

impl Session<Uninitialized> {
    pub fn new(
        NewSessionSpecification {
            user_id,
            current_time,
            expiration_seconds,
        }: NewSessionSpecification,
    ) -> Session<Active> {
        Session {
            id: Identifier::new(),
            user_id,
            state: Active {
                expires_at: current_time + time::Duration::seconds(expiration_seconds as i64),
            },
        }
    }

    pub fn restore(
        RestoreSessionSpecification {
            id,
            user_id,
            revoked_at,
            expires_at,
            current_time,
        }: RestoreSessionSpecification,
    ) -> InitializedSession {
        match revoked_at {
            Some(revoked_at) => InitializedSession::from(Session {
                id: Identifier::from(id.value()),
                user_id,
                state: Revoked { revoked_at },
            }),
            None => match (expires_at - current_time).whole_seconds() > 0 {
                true => InitializedSession::from(Session {
                    id: Identifier::from(id.value()),
                    user_id,
                    state: Active {
                        expires_at: current_time + time::Duration::days(7),
                    },
                }),
                false => InitializedSession::from(Session {
                    id: Identifier::from(id.value()),
                    user_id,
                    state: Expired {
                        expired_at: expires_at,
                    },
                }),
            },
        }
    }
}

impl Session<Active> {
    pub fn revoke(Self { id, user_id, .. }: Self, current_time: UtcDateTime) -> Session<Revoked> {
        Session {
            id: Identifier::from(id.value()),
            user_id,
            state: Revoked {
                revoked_at: current_time,
            },
        }
    }

    pub fn expires_at(&self) -> UtcDateTime {
        self.state.expires_at
    }
}

impl Session<Revoked> {
    pub fn revoked_at(&self) -> UtcDateTime {
        self.state.revoked_at
    }
}

impl Session<Expired> {
    pub fn expired_at(&self) -> UtcDateTime {
        self.state.expired_at
    }
}

impl From<Session<Active>> for InitializedSession {
    fn from(session: Session<Active>) -> Self {
        InitializedSession::Active(session)
    }
}

impl From<Session<Expired>> for InitializedSession {
    fn from(session: Session<Expired>) -> Self {
        InitializedSession::Expired(session)
    }
}

impl From<Session<Revoked>> for InitializedSession {
    fn from(session: Session<Revoked>) -> Self {
        InitializedSession::Revoked(session)
    }
}
