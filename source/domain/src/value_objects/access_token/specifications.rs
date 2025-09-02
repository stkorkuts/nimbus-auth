use time::UtcDateTime;
use ulid::Ulid;

use crate::{entities::user::User, value_objects::identifier::Identifier};

pub struct NewAccessTokenSpecification {
    pub user_id: Identifier<Ulid, User>,
    pub current_time: UtcDateTime,
    pub expiration_seconds: u32,
}

pub struct RestoreAccessTokenSpecification {
    pub signed: String,
    pub secret: String,
    pub current_time: UtcDateTime,
}
