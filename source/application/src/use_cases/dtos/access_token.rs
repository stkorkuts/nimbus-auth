use zeroize::Zeroizing;

pub struct AccessTokenDto {
    pub signed_access_token: Zeroizing<String>,
    pub signed_access_token_expires_at_unix_timestamp: i64,
}
