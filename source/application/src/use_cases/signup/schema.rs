use zeroize::Zeroizing;

use crate::use_cases::UserClaimsDto;

pub struct SignUpRequest<'a> {
    pub user_name: &'a str,
    pub password: &'a Zeroizing<String>,
}

pub struct SignUpResponse {
    pub user: UserClaimsDto,
    pub session_id: String,
    pub signed_access_token: String,
}
