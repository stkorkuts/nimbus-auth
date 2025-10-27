use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use nimbus_auth_application::use_cases::{
    AuthorizationError, AuthorizationRequest, UseCases, UserClaimsDto,
};
use tracing::error;

pub struct Authorization(pub UserClaimsDto);

impl FromRequestParts<UseCases> for Authorization {
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(
        parts: &mut Parts,
        state: &UseCases,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async {
            let signed_token = parts
                .headers
                .get(AUTHORIZATION)
                .ok_or((
                    StatusCode::UNAUTHORIZED,
                    "authorization header is not found",
                ))?
                .to_str()
                .map_err(|_| (StatusCode::BAD_REQUEST, "authorization header is invalid"))?
                .strip_prefix("Bearer ")
                .ok_or((
                    StatusCode::BAD_REQUEST,
                    "authorization header has wrong schema",
                ))?;

            let auth_response = state
                .authorize(AuthorizationRequest { signed_token })
                .await
                .map_err(|err| match err {
                    AuthorizationError::ExtractKeyId(_) => (
                        StatusCode::BAD_REQUEST,
                        "access token does not contain valid key id",
                    ),
                    AuthorizationError::AccessTokenVerification(_) => {
                        (StatusCode::BAD_REQUEST, "access token is invalid")
                    }
                    AuthorizationError::KeyPairNotFound
                    | AuthorizationError::KeyPairExpired
                    | AuthorizationError::KeyPairRevoked => {
                        (StatusCode::BAD_REQUEST, "access token key is invalid")
                    }
                    err => {
                        error!("error in authorization extractor: {err}");
                        (StatusCode::INTERNAL_SERVER_ERROR, "server error")
                    }
                })?;

            Ok(Authorization(auth_response.user))
        }
    }
}
