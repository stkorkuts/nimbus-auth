use zeroize::Zeroizing;

pub struct SessionDto {
    pub session_id: String,
    pub session_expires_at_unix_timestamp: i64,
}
