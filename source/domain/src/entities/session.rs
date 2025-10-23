use std::{borrow::Cow, ops::Deref};

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
        user_claims::{self, UserClaims},
    },
};

pub mod errors;
pub mod specifications;

pub trait SessionState {}

#[derive(Debug, Clone)]
pub struct Active {
    user_claims: UserClaims,
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

impl SessionState for Active {}
impl SessionState for Expired {}
impl SessionState for Revoked {}

#[derive(Debug, Clone)]
pub struct Session<State: SessionState> {
    id: Identifier<Ulid, Session<State>>,
    state: State,
}

#[derive(Debug, Clone)]
pub enum SomeSession<'a> {
    Active(Cow<'a, Session<Active>>),
    Revoked(Cow<'a, Session<Revoked>>),
    Expired(Cow<'a, Session<Expired>>),
}

impl<State: SessionState> Entity<Ulid> for Session<State> {
    type Id = Identifier<Ulid, Session<State>>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl<'a> Entity<Ulid> for SomeSession<'a> {
    type Id = Identifier<Ulid, SomeSession<'a>>;

    fn id(&self) -> &Self::Id {
        match self {
            SomeSession::Active(session) => session.id.as_other_entity_ref(),
            SomeSession::Revoked(session) => session.id.as_other_entity_ref(),
            SomeSession::Expired(session) => session.id.as_other_entity_ref(),
        }
    }
}

impl SomeSession<'_> {
    pub fn new(
        NewSessionSpecification {
            user_claims,
            current_time,
            expiration_seconds: SessionExpirationSeconds(expiration_seconds),
        }: NewSessionSpecification,
    ) -> Session<Active> {
        Session {
            id: Identifier::new(),
            state: Active {
                user_claims,
                expires_at: current_time + time::Duration::seconds(expiration_seconds as i64),
            },
        }
    }

    pub fn restore(
        RestoreSessionSpecification {
            id,
            user_claims,
            revoked_at,
            expires_at,
            current_time,
        }: RestoreSessionSpecification,
    ) -> SomeSession<'static> {
        match revoked_at {
            Some(revoked_at) => SomeSession::from(Session {
                id: id.as_other_entity(),
                state: Revoked { revoked_at },
            }),
            None => match (expires_at - current_time).whole_seconds() > 0 {
                true => SomeSession::from(Session {
                    id: id.as_other_entity(),
                    state: Active {
                        user_claims,
                        expires_at,
                    },
                }),
                false => SomeSession::from(Session {
                    id: id.as_other_entity(),
                    state: Expired {
                        expired_at: expires_at,
                    },
                }),
            },
        }
    }

    pub fn into_owned(self) -> SomeSession<'static> {
        match self {
            SomeSession::Active(cow) => SomeSession::Active(Cow::Owned(cow.into_owned())),
            SomeSession::Revoked(cow) => SomeSession::Revoked(Cow::Owned(cow.into_owned())),
            SomeSession::Expired(cow) => SomeSession::Expired(Cow::Owned(cow.into_owned())),
        }
    }
}

impl<State: SessionState> Deref for Session<State> {
    type Target = State;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl Session<Active> {
    pub fn revoke(self, current_time: OffsetDateTime) -> Session<Revoked> {
        Session {
            id: self.id.as_other_entity(),
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
        let user_claims = self.user_claims.clone();
        (
            Session {
                id: self.id.as_other_entity(),
                state: Revoked {
                    revoked_at: current_time,
                },
            },
            SomeSession::new(NewSessionSpecification {
                user_claims,
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
        AccessToken::new(self.user_claims.clone(), current_time, expiration_seconds)
    }

    pub fn expires_at(&self) -> OffsetDateTime {
        self.expires_at
    }

    pub fn user_claims(&self) -> &UserClaims {
        &self.user_claims
    }
}

impl Session<Revoked> {
    pub fn revoked_at(&self) -> OffsetDateTime {
        self.revoked_at
    }
}

impl Session<Expired> {
    pub fn expired_at(&self) -> OffsetDateTime {
        self.expired_at
    }
}

macro_rules! impl_session_froms {
    ($state:ty, $variant:ident) => {
        impl From<Session<$state>> for SomeSession<'static> {
            fn from(session: Session<$state>) -> Self {
                SomeSession::$variant(Cow::Owned(session))
            }
        }

        impl<'a> From<&'a Session<$state>> for SomeSession<'a> {
            fn from(session: &'a Session<$state>) -> Self {
                SomeSession::$variant(Cow::Borrowed(session))
            }
        }
    };
}

impl_session_froms!(Active, Active);
impl_session_froms!(Expired, Expired);
impl_session_froms!(Revoked, Revoked);
