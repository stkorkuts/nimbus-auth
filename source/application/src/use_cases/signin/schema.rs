use zeroize::Zeroizing;

use crate::use_cases::{
    UserClaimsDto,
    dtos::{access_token::AccessTokenDto, session::SessionDto},
};

pub struct SignInRequest<'a> {
    pub user_name: &'a str,
    pub password: &'a Zeroizing<String>,
}

pub struct SignInResponse {
    pub user: UserClaimsDto,
    pub session: SessionDto,
    pub access_token: AccessTokenDto,
}
