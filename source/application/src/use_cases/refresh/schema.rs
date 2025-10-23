use crate::use_cases::UserClaimsDto;

pub struct RefreshRequest {
    pub session_id: String,
}

pub struct RefreshResponse {
    pub user: UserClaimsDto,
    pub session_id: String,
    pub signed_access_token: String,
}
