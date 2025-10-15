use axum::{
    extract::{FromRequestParts, State},
    http::{StatusCode, request::Parts},
};
use nimbus_auth_application::use_cases::{AuthorizationRequest, UseCases, UserDto};
use tracing::error;

use crate::axum_api::extractors::authorization_extractor::errors::AuthorizationExtractorError;

pub mod errors;

pub struct Authorization(pub Result<UserDto, AuthorizationExtractorError>);

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

            let authorization_result = async {
                let access_token = parts
                    .headers
                    .get("authorization")
                    .ok_or(AuthorizationExtractorError::AuthHeaderIsMissing)?
                    .to_str()
                    .map_err(|_| AuthorizationExtractorError::AuthHeaderContainsNonAscii)?
                    .strip_prefix("Bearer ")
                    .ok_or(AuthorizationExtractorError::AuthHeaderWrongSchema)?;

                let authorized_user = use_cases
                    .authorize(AuthorizationRequest { access_token })
                    .await?;

                Ok::<UserDto, AuthorizationExtractorError>(authorized_user.user)
            }
            .await;

            Ok(Authorization(authorization_result))
        }
    }
}
