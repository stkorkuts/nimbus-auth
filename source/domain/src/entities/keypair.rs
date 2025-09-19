use nimbus_auth_shared::types::AccessTokenExpirationSeconds;
use time::{Duration, OffsetDateTime};
use ulid::Ulid;

use crate::{
    entities::{
        Entity,
        keypair::{
            specifications::{NewKeyPairSpecification, RestoreKeyPairSpecification},
            value_objects::KeyPairValue,
        },
    },
    value_objects::identifier::{Identifier, IdentifierOfType},
};

pub mod errors;
pub mod specifications;
pub mod value_objects;

pub trait KeyPairState {}

#[derive(Debug, Clone)]
pub struct Active {
    value: KeyPairValue,
}

#[derive(Debug, Clone)]
pub struct Expiring {
    value: KeyPairValue,
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
pub struct KeyPair<State: KeyPairState> {
    id: Identifier<Ulid, KeyPair<State>>,
    state: State,
}

pub enum SomeKeyPair {
    Active {
        id: Identifier<Ulid, SomeKeyPair>,
        keypair: KeyPair<Active>,
    },
    Expiring {
        id: Identifier<Ulid, SomeKeyPair>,
        keypair: KeyPair<Expiring>,
    },
    Expired {
        id: Identifier<Ulid, SomeKeyPair>,
        keypair: KeyPair<Expired>,
    },
    Revoked {
        id: Identifier<Ulid, SomeKeyPair>,
        keypair: KeyPair<Revoked>,
    },
}

pub enum SomeKeyPairRef<'a> {
    Active(&'a KeyPair<Active>),
    Expiring(&'a KeyPair<Expiring>),
    Expired(&'a KeyPair<Expired>),
    Revoked(&'a KeyPair<Revoked>),
}

impl<State: KeyPairState> Entity<Ulid> for KeyPair<State> {
    type Id = Identifier<Ulid, KeyPair<State>>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Entity<Ulid> for SomeKeyPair {
    type Id = Identifier<Ulid, SomeKeyPair>;

    fn id(&self) -> &Self::Id {
        match self {
            SomeKeyPair::Active { id, .. } => id,
            SomeKeyPair::Expiring { id, .. } => id,
            SomeKeyPair::Revoked { id, .. } => id,
            SomeKeyPair::Expired { id, .. } => id,
        }
    }
}

impl KeyPairState for Active {}
impl KeyPairState for Expiring {}
impl KeyPairState for Expired {}
impl KeyPairState for Revoked {}

impl SomeKeyPair {
    pub fn new(NewKeyPairSpecification { value }: NewKeyPairSpecification) -> KeyPair<Active> {
        KeyPair {
            id: Identifier::new(),
            state: Active { value },
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
    ) -> SomeKeyPair {
        match revoked_at {
            Some(revoked_at) => SomeKeyPair::from(KeyPair {
                id: Identifier::from(*id.value()),
                state: Revoked { revoked_at },
            }),
            None => match expires_at {
                None => SomeKeyPair::from(KeyPair {
                    id: Identifier::from(*id.value()),
                    state: Active { value },
                }),
                Some(expires_at) => match (expires_at - current_time).whole_seconds() > 0 {
                    true => SomeKeyPair::from(KeyPair {
                        id: Identifier::from(*id.value()),
                        state: Expiring { expires_at, value },
                    }),
                    false => SomeKeyPair::from(KeyPair {
                        id: Identifier::from(*id.value()),
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
            id: Identifier::from(*self.id.value()),
            state: Revoked {
                revoked_at: current_time,
            },
        }
    }

    /// Rotates a key
    ///
    /// Current key moves to the `Expiring` status with expiration time equals to `expiration_seconds * 2`
    ///
    /// New key is generated in `Active` status without expiration time
    pub fn rotate<'a>(
        self,
        value: KeyPairValue,
        current_time: OffsetDateTime,
        expiration_seconds: AccessTokenExpirationSeconds,
    ) -> (KeyPair<Expiring>, KeyPair<Active>) {
        (
            KeyPair {
                id: Identifier::from(*self.id.value()),
                state: Expiring {
                    value: self.state.value,
                    expires_at: current_time + Duration::seconds((expiration_seconds.0 * 2) as i64),
                },
            },
            SomeKeyPair::new(NewKeyPairSpecification { value }),
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

impl From<KeyPair<Active>> for SomeKeyPair {
    fn from(keypair: KeyPair<Active>) -> Self {
        SomeKeyPair::Active {
            id: Identifier::from(*keypair.id().value()),
            keypair,
        }
    }
}

impl From<KeyPair<Expiring>> for SomeKeyPair {
    fn from(keypair: KeyPair<Expiring>) -> Self {
        SomeKeyPair::Expiring {
            id: Identifier::from(*keypair.id().value()),
            keypair,
        }
    }
}

impl From<KeyPair<Expired>> for SomeKeyPair {
    fn from(keypair: KeyPair<Expired>) -> Self {
        SomeKeyPair::Expired {
            id: Identifier::from(*keypair.id().value()),
            keypair,
        }
    }
}

impl From<KeyPair<Revoked>> for SomeKeyPair {
    fn from(keypair: KeyPair<Revoked>) -> Self {
        SomeKeyPair::Revoked {
            id: Identifier::from(*keypair.id().value()),
            keypair,
        }
    }
}

impl<'a> From<&'a KeyPair<Active>> for SomeKeyPairRef<'a> {
    fn from(keypair: &'a KeyPair<Active>) -> Self {
        SomeKeyPairRef::Active(keypair)
    }
}

impl<'a> From<&'a KeyPair<Expiring>> for SomeKeyPairRef<'a> {
    fn from(keypair: &'a KeyPair<Expiring>) -> Self {
        SomeKeyPairRef::Expiring(keypair)
    }
}

impl<'a> From<&'a KeyPair<Expired>> for SomeKeyPairRef<'a> {
    fn from(keypair: &'a KeyPair<Expired>) -> Self {
        SomeKeyPairRef::Expired(keypair)
    }
}

impl<'a> From<&'a KeyPair<Revoked>> for SomeKeyPairRef<'a> {
    fn from(keypair: &'a KeyPair<Revoked>) -> Self {
        SomeKeyPairRef::Revoked(keypair)
    }
}
