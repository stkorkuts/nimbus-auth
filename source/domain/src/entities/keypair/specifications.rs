use time::OffsetDateTime;
use ulid::Ulid;

use crate::{
    entities::keypair::{SomeKeyPair, value_objects::KeyPairValue},
    value_objects::identifier::Identifier,
};

pub struct NewKeyPairSpecification {
    pub value: KeyPairValue,
}

pub struct RestoreKeyPairSpecification<'a> {
    pub id: Identifier<Ulid, SomeKeyPair<'a>>,
    pub value: KeyPairValue,
    pub expires_at: Option<OffsetDateTime>,
    pub revoked_at: Option<OffsetDateTime>,
    pub current_time: OffsetDateTime,
}
