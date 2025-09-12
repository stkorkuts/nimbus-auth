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

pub struct Uninitialized {}

pub struct Active {
    expires_at: OffsetDateTime,
}

pub struct Expired {
    expired_at: OffsetDateTime,
}

pub struct Revoked {
    revoked_at: OffsetDateTime,
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

pub enum InitializedSessionRef<'a> {
    Active(&'a Session<Active>),
    Revoked(&'a Session<Revoked>),
    Expired(&'a Session<Expired>),
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
            expiration_seconds: SessionExpirationSeconds(expiration_seconds),
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
                    state: Active { expires_at },
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
    pub fn revoke(
        Self { id, user_id, .. }: Self,
        current_time: OffsetDateTime,
    ) -> Session<Revoked> {
        Session {
            id: Identifier::from(id.value()),
            user_id,
            state: Revoked {
                revoked_at: current_time,
            },
        }
    }

    pub fn refresh(
        Self { id, user_id, .. }: Self,
        current_time: OffsetDateTime,
        expiration_seconds: SessionExpirationSeconds,
    ) -> (Session<Revoked>, Session<Active>) {
        (
            Session {
                id: Identifier::from(id.value()),
                user_id: user_id.clone(),
                state: Revoked {
                    revoked_at: current_time,
                },
            },
            Session::<Uninitialized>::new(NewSessionSpecification {
                user_id: user_id.clone(),
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
        AccessToken::new(self.user_id.clone(), current_time, expiration_seconds)
    }

    pub fn expires_at(&self) -> OffsetDateTime {
        self.state.expires_at
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

impl<'a> From<&'a Session<Active>> for InitializedSessionRef<'a> {
    fn from(session: &'a Session<Active>) -> Self {
        InitializedSessionRef::Active(session)
    }
}

impl<'a> From<&'a Session<Expired>> for InitializedSessionRef<'a> {
    fn from(session: &'a Session<Expired>) -> Self {
        InitializedSessionRef::Expired(session)
    }
}

impl<'a> From<&'a Session<Revoked>> for InitializedSessionRef<'a> {
    fn from(session: &'a Session<Revoked>) -> Self {
        InitializedSessionRef::Revoked(session)
    }
}
