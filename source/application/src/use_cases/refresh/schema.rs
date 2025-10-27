use crate::use_cases::{
    UserClaimsDto,
    dtos::{access_token::AccessTokenDto, session::SessionDto},
};

pub struct RefreshRequest<'a> {
    pub session_id: &'a str,
}

pub struct RefreshResponse {
    pub user: UserClaimsDto,
    pub session: SessionDto,
    pub access_token: AccessTokenDto,
}
