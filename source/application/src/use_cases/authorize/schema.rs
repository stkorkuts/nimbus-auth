use crate::use_cases::UserDto;

pub struct AuthorizationRequest<'a> {
    pub access_token: &'a str,
}

pub struct AuthorizationResponse {
    pub user: UserDto,
}
