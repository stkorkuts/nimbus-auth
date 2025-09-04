use nimbus_auth_shared::config::AccessTokenExpirationSeconds;
use time::{Duration, OffsetDateTime};
use ulid::Ulid;

use crate::{
    entities::{
        Entity,
        keypair::{
            errors::KeyPairError,
            specifications::{NewKeyPairSpecification, RestoreKeyPairSpecification},
        },
    },
    value_objects::{
        identifier::{Identifier, IdentifierOfType},
        keypair_value::KeyPairValue,
    },
};

pub mod errors;
pub mod specifications;

pub trait KeyPairState {}

pub struct Uninitialized {}

pub struct Active {
    value: KeyPairValue,
}

pub struct Expiring {
    value: KeyPairValue,
    expires_at: OffsetDateTime,
}

pub struct Expired {
    expired_at: OffsetDateTime,
}

pub struct Revoked {
    revoked_at: OffsetDateTime,
}

pub struct KeyPair<State: KeyPairState> {
    id: Identifier<Ulid, KeyPair<State>>,
    state: State,
}

pub enum InitializedKeyPair {
    Active(KeyPair<Active>),
    Expiring(KeyPair<Expiring>),
    Expired(KeyPair<Expired>),
    Revoked(KeyPair<Revoked>),
}

impl<State: KeyPairState> Entity<Ulid> for KeyPair<State> {
    type Id = Identifier<Ulid, KeyPair<State>>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl KeyPairState for Uninitialized {}
impl KeyPairState for Active {}
impl KeyPairState for Expiring {}
impl KeyPairState for Expired {}
impl KeyPairState for Revoked {}

impl KeyPair<Uninitialized> {
    pub fn new(NewKeyPairSpecification {}: NewKeyPairSpecification) -> KeyPair<Active> {
        KeyPair {
            id: Identifier::new(),
            state: Active {
                value: KeyPairValue::new(),
            },
        }
    }

    pub fn restore(
        RestoreKeyPairSpecification {
            id,
            value,
            expires_at,
            revoked_at,
            current_time,
        }: RestoreKeyPairSpecification,
    ) -> InitializedKeyPair {
        match revoked_at {
            Some(revoked_at) => InitializedKeyPair::from(KeyPair {
                id: Identifier::from(id.value()),
                state: Revoked { revoked_at },
            }),
            None => match expires_at {
                None => InitializedKeyPair::from(KeyPair {
                    id: Identifier::from(id.value()),
                    state: Active { value },
                }),
                Some(expires_at) => match (expires_at - current_time).whole_seconds() > 0 {
                    true => InitializedKeyPair::from(KeyPair {
                        id: Identifier::from(id.value()),
                        state: Expiring { expires_at, value },
                    }),
                    false => InitializedKeyPair::from(KeyPair {
                        id: Identifier::from(id.value()),
                        state: Expired {
                            expired_at: expires_at,
                        },
                    }),
                },
            },
        }
    }
}

impl KeyPair<Active> {
    pub fn value(&self) -> &KeyPairValue {
        &self.state.value
    }

    pub fn revoke(self, current_time: OffsetDateTime) -> KeyPair<Revoked> {
        KeyPair {
            id: Identifier::from(self.id.value()),
            state: Revoked {
                revoked_at: current_time,
            },
        }
    }

    pub fn rotate(
        self,
        current_time: OffsetDateTime,
        expiration_seconds: AccessTokenExpirationSeconds,
    ) -> (KeyPair<Expiring>, KeyPair<Active>) {
        (
            KeyPair {
                id: Identifier::from(self.id.value()),
                state: Expiring {
                    value: self.state.value,
                    expires_at: current_time + Duration::seconds(expiration_seconds.0 as i64),
                },
            },
            KeyPair::<Uninitialized>::new(NewKeyPairSpecification {}),
        )
    }
}

impl KeyPair<Expiring> {
    pub fn value(&self) -> &KeyPairValue {
        &self.state.value
    }

    pub fn expires_at(&self) -> OffsetDateTime {
        self.state.expires_at
    }
}

impl KeyPair<Expired> {
    pub fn expired_at(&self) -> OffsetDateTime {
        self.state.expired_at
    }
}

impl From<KeyPair<Active>> for InitializedKeyPair {
    fn from(session: KeyPair<Active>) -> Self {
        InitializedKeyPair::Active(session)
    }
}

impl From<KeyPair<Expiring>> for InitializedKeyPair {
    fn from(session: KeyPair<Expiring>) -> Self {
        InitializedKeyPair::Expiring(session)
    }
}

impl From<KeyPair<Expired>> for InitializedKeyPair {
    fn from(session: KeyPair<Expired>) -> Self {
        InitializedKeyPair::Expired(session)
    }
}

impl From<KeyPair<Revoked>> for InitializedKeyPair {
    fn from(session: KeyPair<Revoked>) -> Self {
        InitializedKeyPair::Revoked(session)
    }
}
