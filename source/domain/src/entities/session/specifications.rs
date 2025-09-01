use time::UtcDateTime;

pub struct RestoreSessionSpecification {
    pub value: String,
    pub revoked_at: Option<UtcDateTime>,
    pub expires_at: UtcDateTime,
    pub current_time: UtcDateTime,
}
