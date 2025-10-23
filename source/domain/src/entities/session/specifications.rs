use nimbus_auth_shared::types::SessionExpirationSeconds;
use time::OffsetDateTime;
use ulid::Ulid;

use crate::{
    entities::session::SomeSession,
    value_objects::{identifier::Identifier, user_claims::UserClaims},
};

pub struct NewSessionSpecification {
    pub user_claims: UserClaims,
    pub current_time: OffsetDateTime,
    pub expiration_seconds: SessionExpirationSeconds,
}

pub struct RestoreSessionSpecification<'a> {
    pub id: Identifier<Ulid, SomeSession<'a>>,
    pub user_claims: UserClaims,
    pub revoked_at: Option<OffsetDateTime>,
    pub expires_at: OffsetDateTime,
    pub current_time: OffsetDateTime,
}
