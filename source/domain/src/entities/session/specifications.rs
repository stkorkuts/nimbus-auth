use time::UtcDateTime;
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
    pub current_time: UtcDateTime,
    pub expiration_seconds: u32,
}

pub struct RestoreSessionSpecification {
    pub id: Identifier<Ulid, Session<Uninitialized>>,
    pub user_id: Identifier<Ulid, User>,
    pub revoked_at: Option<UtcDateTime>,
    pub expires_at: UtcDateTime,
    pub current_time: UtcDateTime,
}
