use crate::use_cases::{
    AuthorizationRequest, AuthorizationResponse, authorize::errors::AuthorizationError,
};

pub mod errors;
pub mod schema;

pub async fn handle_authorize<'a>(
    AuthorizationRequest { access_token }: AuthorizationRequest<'a>,
) -> Result<AuthorizationResponse, AuthorizationError> {
    todo!()
}
