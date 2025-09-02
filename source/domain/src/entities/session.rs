pub mod errors;
pub mod specifications;

use time::UtcDateTime;
use ulid::Ulid;

use crate::{
    entities::{
        Entity,
        session::specifications::{NewSessionSpecification, RestoreSessionSpecification},
    },
    value_objects::{Identifier, IdentifierOfType},
};

pub trait SessionState {}

pub struct Initial {}

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
    state: State,
}

pub enum OneOfSession {
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

impl SessionState for Initial {}
impl SessionState for Active {}
impl SessionState for Expired {}
impl SessionState for Revoked {}

impl Session<Initial> {
    pub fn new(specs: NewSessionSpecification) -> Session<Active> {
        Session {
            id: Identifier::new(),
            state: Active {
                expires_at: specs.current_time + time::Duration::days(7),
            },
        }
    }

    pub fn restore(specs: RestoreSessionSpecification) -> OneOfSession {
        match specs.revoked_at {
            Some(revoked_at) => OneOfSession::from(Session {
                id: Identifier::from(specs.id),
                state: Revoked { revoked_at },
            }),
            None => match (specs.expires_at - specs.current_time).whole_seconds() > 0 {
                true => OneOfSession::from(Session {
                    id: Identifier::from(specs.id),
                    state: Active {
                        expires_at: specs.current_time + time::Duration::days(7),
                    },
                }),
                false => OneOfSession::from(Session {
                    id: Identifier::from(specs.id),
                    state: Expired {
                        expired_at: specs.expires_at,
                    },
                }),
            },
        }
    }
}

impl Session<Active> {
    pub fn revoke(self, current_time: UtcDateTime) -> Session<Revoked> {
        Session {
            id: Identifier::from(self.id.value()),
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

impl From<Session<Active>> for OneOfSession {
    fn from(session: Session<Active>) -> Self {
        OneOfSession::Active(session)
    }
}

impl From<Session<Expired>> for OneOfSession {
    fn from(session: Session<Expired>) -> Self {
        OneOfSession::Expired(session)
    }
}

impl From<Session<Revoked>> for OneOfSession {
    fn from(session: Session<Revoked>) -> Self {
        OneOfSession::Revoked(session)
    }
}
