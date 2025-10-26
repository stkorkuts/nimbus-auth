use crate::use_cases::{
    UserClaimsDto,
    dtos::{access_token::AccessTokenDto, session::SessionDto},
};

pub struct RefreshRequest {
    pub session_id: String,
}

pub struct RefreshResponse {
    pub user: UserClaimsDto,
    pub session: SessionDto,
    pub access_token: AccessTokenDto,
}
