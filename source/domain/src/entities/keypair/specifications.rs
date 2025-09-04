use time::OffsetDateTime;
use ulid::Ulid;

use crate::{
    entities::keypair::{KeyPair, Uninitialized},
    value_objects::{identifier::Identifier, keypair_value::KeyPairValue},
};

pub struct NewKeyPairSpecification {}

pub struct RestoreKeyPairSpecification {
    pub id: Identifier<Ulid, KeyPair<Uninitialized>>,
    pub value: KeyPairValue,
    pub expires_at: Option<OffsetDateTime>,
    pub revoked_at: Option<OffsetDateTime>,
    pub current_time: OffsetDateTime,
}
