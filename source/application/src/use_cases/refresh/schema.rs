use crate::use_cases::UserDto;

pub struct RefreshRequest {
    pub session_id: String,
}

pub struct RefreshResponse {
    pub user: UserDto,
    pub session_id: String,
    pub signed_access_token: String,
}
