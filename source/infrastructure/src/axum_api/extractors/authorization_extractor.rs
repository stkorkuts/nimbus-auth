use axum::{
    extract::{FromRequestParts, State},
    http::{StatusCode, request::Parts},
};
use nimbus_auth_application::use_cases::{AuthorizationRequest, UseCases, UserDto};
use tracing::error;

pub struct Authorization(pub UserDto);

impl FromRequestParts<UseCases> for Authorization {
    type Rejection = (StatusCode, &'static str);

    #[doc = " Perform the extraction."]
    fn from_request_parts(
        parts: &mut Parts,
        state: &UseCases,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async {
            let State(use_cases) = State::<UseCases>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    error!("can not get use_cases from request parts in Authorization extractor");
                    (StatusCode::INTERNAL_SERVER_ERROR, "")
                })?;

            let access_token = parts
                .headers
                .get("authorization")
                .ok_or((
                    StatusCode::UNAUTHORIZED,
                    "Provide authorization header with value",
                ))?
                .to_str()
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        "Wrong authorization header value format",
                    )
                })?
                .strip_prefix("Bearer ")
                .ok_or({
                    (
                        StatusCode::UNAUTHORIZED,
                        "Wrong authorization header schema",
                    )
                })?;

            let authorized_user = use_cases
                .authorize(AuthorizationRequest { access_token })
                .await
                .map_err(|err| {
                    error!("error while trying to authorize a user: {err}");
                    (
                        StatusCode::UNAUTHORIZED,
                        "error while trying to authorize a user",
                    )
                })?;

            Ok(Authorization(authorized_user.user))
        }
    }
}
