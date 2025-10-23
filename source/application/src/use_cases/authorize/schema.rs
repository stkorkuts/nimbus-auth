use crate::use_cases::UserClaimsDto;

pub struct AuthorizationRequest<'a> {
    pub signed_token: &'a str,
}

pub struct AuthorizationResponse {
    pub user: UserClaimsDto,
}
