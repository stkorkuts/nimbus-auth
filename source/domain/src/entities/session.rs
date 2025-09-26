use nimbus_auth_shared::types::{AccessTokenExpirationSeconds, SessionExpirationSeconds};
use time::OffsetDateTime;
use ulid::Ulid;

use crate::{
    entities::{
        Entity,
        session::specifications::{NewSessionSpecification, RestoreSessionSpecification},
        user::User,
    },
    value_objects::{
        access_token::AccessToken,
        identifier::{Identifier, IdentifierOfType},
    },
};

pub mod errors;
pub mod specifications;

pub trait SessionState {}

#[derive(Debug, Clone)]
pub struct Active {
    user_id: Identifier<Ulid, User>,
    expires_at: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct Expired {
    expired_at: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct Revoked {
    revoked_at: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct Session<State: SessionState> {
    id: Identifier<Ulid, Session<State>>,
    state: State,
}

#[derive(Debug, Clone)]
pub enum SomeSession {
    Active {
        id: Identifier<Ulid, SomeSession>,
        session: Session<Active>,
    },
    Revoked {
        id: Identifier<Ulid, SomeSession>,
        session: Session<Revoked>,
    },
    Expired {
        id: Identifier<Ulid, SomeSession>,
        session: Session<Expired>,
    },
}

#[derive(Debug, Clone)]
pub enum SomeSessionRef<'a> {
    Active(&'a Session<Active>),
    Revoked(&'a Session<Revoked>),
    Expired(&'a Session<Expired>),
}

impl<'a> SomeSessionRef<'a> {
    pub fn deref_clone(&self) -> SomeSession {
        match self.clone() {
            SomeSessionRef::Active(session_ref) => SomeSession::Active {
                id: Identifier::from(*session_ref.id().value()),
                session: session_ref.clone(),
            },
            SomeSessionRef::Revoked(session_ref) => SomeSession::Revoked {
                id: Identifier::from(*session_ref.id().value()),
                session: session_ref.clone(),
            },
            SomeSessionRef::Expired(session_ref) => SomeSession::Expired {
                id: Identifier::from(*session_ref.id().value()),
                session: session_ref.clone(),
            },
        }
    }
}

impl<State: SessionState> Entity<Ulid> for Session<State> {
    type Id = Identifier<Ulid, Session<State>>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Entity<Ulid> for SomeSession {
    type Id = Identifier<Ulid, SomeSession>;

    fn id(&self) -> &Self::Id {
        match self {
            SomeSession::Active { id, .. } => id,
            SomeSession::Revoked { id, .. } => id,
            SomeSession::Expired { id, .. } => id,
        }
    }
}

impl SessionState for Active {}
impl SessionState for Expired {}
impl SessionState for Revoked {}

impl SomeSession {
    pub fn new(
        NewSessionSpecification {
            user_id,
            current_time,
            expiration_seconds: SessionExpirationSeconds(expiration_seconds),
        }: NewSessionSpecification,
    ) -> Session<Active> {
        Session {
            id: Identifier::new(),
            state: Active {
                user_id,
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
    ) -> SomeSession {
        match revoked_at {
            Some(revoked_at) => SomeSession::from(Session {
                id: Identifier::from(*id.value()),
                state: Revoked { revoked_at },
            }),
            None => match (expires_at - current_time).whole_seconds() > 0 {
                true => SomeSession::from(Session {
                    id: Identifier::from(*id.value()),
                    state: Active {
                        user_id,
                        expires_at,
                    },
                }),
                false => SomeSession::from(Session {
                    id: Identifier::from(*id.value()),
                    state: Expired {
                        expired_at: expires_at,
                    },
                }),
            },
        }
    }
}

impl Session<Active> {
    pub fn revoke(self, current_time: OffsetDateTime) -> Session<Revoked> {
        Session {
            id: Identifier::from(*self.id.value()),
            state: Revoked {
                revoked_at: current_time,
            },
        }
    }

    pub fn refresh(
        self,
        current_time: OffsetDateTime,
        expiration_seconds: SessionExpirationSeconds,
    ) -> (Session<Revoked>, Session<Active>) {
        (
            Session {
                id: Identifier::from(*self.id.value()),
                state: Revoked {
                    revoked_at: current_time,
                },
            },
            SomeSession::new(NewSessionSpecification {
                user_id: self.state.user_id.clone(),
                current_time,
                expiration_seconds,
            }),
        )
    }

    pub fn generate_access_token(
        &self,
        current_time: OffsetDateTime,
        expiration_seconds: AccessTokenExpirationSeconds,
    ) -> AccessToken {
        AccessToken::new(self.state.user_id.clone(), current_time, expiration_seconds)
    }

    pub fn expires_at(&self) -> OffsetDateTime {
        self.state.expires_at
    }

    pub fn user_id(&self) -> &Identifier<Ulid, User> {
        &self.state.user_id
    }
}

impl Session<Revoked> {
    pub fn revoked_at(&self) -> OffsetDateTime {
        self.state.revoked_at
    }
}

impl Session<Expired> {
    pub fn expired_at(&self) -> OffsetDateTime {
        self.state.expired_at
    }
}

impl From<Session<Active>> for SomeSession {
    fn from(session: Session<Active>) -> Self {
        SomeSession::Active {
            id: Identifier::from(*session.id().value()),
            session,
        }
    }
}

impl From<Session<Expired>> for SomeSession {
    fn from(session: Session<Expired>) -> Self {
        SomeSession::Expired {
            id: Identifier::from(*session.id().value()),
            session,
        }
    }
}

impl From<Session<Revoked>> for SomeSession {
    fn from(session: Session<Revoked>) -> Self {
        SomeSession::Revoked {
            id: Identifier::from(*session.id().value()),
            session,
        }
    }
}

impl<'a> From<&'a Session<Active>> for SomeSessionRef<'a> {
    fn from(session: &'a Session<Active>) -> Self {
        SomeSessionRef::Active(session)
    }
}

impl<'a> From<&'a Session<Expired>> for SomeSessionRef<'a> {
    fn from(session: &'a Session<Expired>) -> Self {
        SomeSessionRef::Expired(session)
    }
}

impl<'a> From<&'a Session<Revoked>> for SomeSessionRef<'a> {
    fn from(session: &'a Session<Revoked>) -> Self {
        SomeSessionRef::Revoked(session)
    }
}
