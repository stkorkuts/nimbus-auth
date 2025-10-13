use std::{borrow::Cow, ops::Deref};

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

impl KeyPairState for Active {}
impl KeyPairState for Expiring {}
impl KeyPairState for Expired {}
impl KeyPairState for Revoked {}

#[derive(Debug, Clone)]
pub struct KeyPair<State: KeyPairState> {
    id: Identifier<Ulid, KeyPair<State>>,
    state: State,
}

#[derive(Debug, Clone)]
pub enum SomeKeyPair<'a> {
    Active(Cow<'a, KeyPair<Active>>),
    Expiring(Cow<'a, KeyPair<Expiring>>),
    Expired(Cow<'a, KeyPair<Expired>>),
    Revoked(Cow<'a, KeyPair<Revoked>>),
}

impl<State: KeyPairState> Entity<Ulid> for KeyPair<State> {
    type Id = Identifier<Ulid, KeyPair<State>>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl<'a> Entity<Ulid> for SomeKeyPair<'a> {
    type Id = Identifier<Ulid, SomeKeyPair<'a>>;

    fn id(&self) -> &Self::Id {
        match self {
            SomeKeyPair::Active(keypair) => keypair.id.as_other_entity_ref(),
            SomeKeyPair::Expiring(keypair) => keypair.id.as_other_entity_ref(),
            SomeKeyPair::Revoked(keypair) => keypair.id.as_other_entity_ref(),
            SomeKeyPair::Expired(keypair) => keypair.id.as_other_entity_ref(),
        }
    }
}

impl SomeKeyPair<'_> {
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
    ) -> SomeKeyPair<'static> {
        match revoked_at {
            Some(revoked_at) => SomeKeyPair::from(KeyPair {
                id: Identifier::from(*id.value()),
                state: Revoked { revoked_at },
            }),
            None => match expires_at {
                None => SomeKeyPair::from(KeyPair {
                    id: id.as_other_entity(),
                    state: Active { value },
                }),
                Some(expires_at) => match (expires_at - current_time).whole_seconds() > 0 {
                    true => SomeKeyPair::from(KeyPair {
                        id: id.as_other_entity(),
                        state: Expiring { expires_at, value },
                    }),
                    false => SomeKeyPair::from(KeyPair {
                        id: id.as_other_entity(),
                        state: Expired {
                            expired_at: expires_at,
                        },
                    }),
                },
            },
        }
    }

    pub fn into_owned(self) -> SomeKeyPair<'static> {
        match self {
            SomeKeyPair::Active(cow) => SomeKeyPair::Active(Cow::Owned(cow.into_owned())),
            SomeKeyPair::Expiring(cow) => SomeKeyPair::Expiring(Cow::Owned(cow.into_owned())),
            SomeKeyPair::Revoked(cow) => SomeKeyPair::Revoked(Cow::Owned(cow.into_owned())),
            SomeKeyPair::Expired(cow) => SomeKeyPair::Expired(Cow::Owned(cow.into_owned())),
        }
    }
}

impl<State: KeyPairState> Deref for KeyPair<State> {
    type Target = State;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl KeyPair<Active> {
    pub fn value(&self) -> &KeyPairValue {
        &self.state.value
    }

    pub fn revoke(self, current_time: OffsetDateTime) -> KeyPair<Revoked> {
        KeyPair {
            id: self.id.as_other_entity(),
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
                id: self.id.as_other_entity(),
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

macro_rules! impl_keypair_froms {
    ($state:ty, $variant:ident) => {
        impl From<KeyPair<$state>> for SomeKeyPair<'static> {
            fn from(session: KeyPair<$state>) -> Self {
                SomeKeyPair::$variant(Cow::Owned(session))
            }
        }

        impl<'a> From<&'a KeyPair<$state>> for SomeKeyPair<'a> {
            fn from(session: &'a KeyPair<$state>) -> Self {
                SomeKeyPair::$variant(Cow::Borrowed(session))
            }
        }
    };
}

impl_keypair_froms!(Active, Active);
impl_keypair_froms!(Expiring, Expiring);
impl_keypair_froms!(Expired, Expired);
impl_keypair_froms!(Revoked, Revoked);
