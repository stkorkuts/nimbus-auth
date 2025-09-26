use zeroize::Zeroizing;

use crate::use_cases::UserDto;

pub struct SignInRequest<'a> {
    pub user_name: &'a str,
    pub password: &'a Zeroizing<String>,
}

pub struct SignInResponse {
    pub user: UserDto,
    pub session_id: String,
    pub signed_access_token: String,
}
