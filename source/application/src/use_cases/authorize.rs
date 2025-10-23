use std::sync::Arc;

use nimbus_auth_domain::{
    entities::keypair::SomeKeyPair, value_objects::access_token::AccessToken,
};

use crate::{
    services::keypair_repository::KeyPairRepository,
    use_cases::{
        AuthorizationRequest, AuthorizationResponse, UserClaimsDto,
        authorize::errors::AuthorizationError,
    },
};

pub mod errors;
pub mod schema;

pub async fn handle_authorize<'a>(
    AuthorizationRequest { signed_token }: AuthorizationRequest<'a>,
    keypair_repository: Arc<dyn KeyPairRepository>,
) -> Result<AuthorizationResponse, AuthorizationError> {
    let keypair_id = AccessToken::extract_keypair_id(signed_token)?;
    let keypair = keypair_repository
        .get_by_id(keypair_id.as_other_entity_ref())
        .await?
        .ok_or(AuthorizationError::KeyPairNotFound)?;

    let access_token = match keypair {
        SomeKeyPair::Active(active) => AccessToken::verify_with_active(signed_token, &active),
        SomeKeyPair::Expiring(expiring) => {
            AccessToken::verify_with_expiring(signed_token, &expiring)
        }
        SomeKeyPair::Expired(_) => return Err(AuthorizationError::KeyPairExpired),
        SomeKeyPair::Revoked(_) => return Err(AuthorizationError::KeyPairRevoked),
    }?;

    Ok(AuthorizationResponse {
        user: UserClaimsDto::from(access_token.user_claims()),
    })
}
