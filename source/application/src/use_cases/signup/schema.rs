use crate::use_cases::UserDto;

pub struct SignUpRequest<'a> {
    pub user_name: &'a str,
    pub password: &'a str,
}

pub struct SignUpResponse {
    pub user: UserDto,
    pub session_id: String,
    pub signed_access_token: String,
}
