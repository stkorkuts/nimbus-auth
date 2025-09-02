use time::UtcDateTime;
use ulid::Ulid;

pub struct NewSessionSpecification {
    pub current_time: UtcDateTime,
}

pub struct RestoreSessionSpecification {
    pub id: Ulid,
    pub revoked_at: Option<UtcDateTime>,
    pub expires_at: UtcDateTime,
    pub current_time: UtcDateTime,
}
