use nimbus_auth_shared::types::SessionExpirationSeconds;
use time::OffsetDateTime;
use ulid::Ulid;

use crate::{
    entities::{
        session::{Session, Uninitialized},
        user::User,
    },
    value_objects::identifier::Identifier,
};

pub struct NewSessionSpecification {
    pub user_id: Identifier<Ulid, User>,
    pub current_time: OffsetDateTime,
    pub expiration_seconds: SessionExpirationSeconds,
}

pub struct RestoreSessionSpecification {
    pub id: Identifier<Ulid, Session<Uninitialized>>,
    pub user_id: Identifier<Ulid, User>,
    pub revoked_at: Option<OffsetDateTime>,
    pub expires_at: OffsetDateTime,
    pub current_time: OffsetDateTime,
}
