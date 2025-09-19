use std::{str::FromStr, sync::Arc};

use nimbus_auth_domain::{
    entities::keypair::{self, SomeKeyPair},
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::errors::ErrorBoxed;
use ulid::Ulid;

use crate::{
    services::keypair_repository::KeyPairRepository,
    use_cases::{GetPublicKeyError, GetPublicKeyRequest, GetPublicKeyResponse},
};

pub mod errors;
pub mod schema;

pub async fn handle_get_public_key<'a>(
    GetPublicKeyRequest { key_id }: GetPublicKeyRequest<'a>,
    keypair_repository: Arc<dyn KeyPairRepository>,
) -> Result<GetPublicKeyResponse, GetPublicKeyError> {
    let keypair = keypair_repository
        .get_by_id(&Identifier::from(
            Ulid::from_str(key_id).map_err(ErrorBoxed::from)?,
        ))
        .await?
        .ok_or(GetPublicKeyError::KeyPairNotFound)?;

    Ok(GetPublicKeyResponse {
        public_key_pem: match keypair {
            SomeKeyPair::Active { keypair, .. } => keypair.value().public_key_pem().to_vec(),
            SomeKeyPair::Expiring { keypair, .. } => keypair.value().public_key_pem().to_vec(),
            SomeKeyPair::Revoked { .. } => return Err(GetPublicKeyError::KeyPairIsRevoked),
            SomeKeyPair::Expired { .. } => return Err(GetPublicKeyError::KeyPairIsExpired),
        },
    })
}
