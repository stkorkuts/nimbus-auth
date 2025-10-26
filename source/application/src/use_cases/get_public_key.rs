use std::{str::FromStr, sync::Arc};

use nimbus_auth_domain::{entities::keypair::SomeKeyPair, value_objects::identifier::Identifier};
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
    let keypair = match key_id {
        Some(key_id) => keypair_repository
            .get_by_id(&Identifier::from(
                Ulid::from_str(key_id).map_err(ErrorBoxed::from)?,
            ))
            .await?
            .ok_or(GetPublicKeyError::KeyPairNotFound)?,
        None => SomeKeyPair::from(
            keypair_repository
                .get_active()
                .await?
                .ok_or(GetPublicKeyError::KeyPairNotFound)?,
        ),
    };

    Ok(GetPublicKeyResponse {
        public_key_pem: match keypair {
            SomeKeyPair::Active(keypair) => keypair.value().public_key_pem(),
            SomeKeyPair::Expiring(keypair) => keypair.value().public_key_pem(),
            SomeKeyPair::Revoked(_) => return Err(GetPublicKeyError::KeyPairIsRevoked),
            SomeKeyPair::Expired(_) => return Err(GetPublicKeyError::KeyPairIsExpired),
        },
    })
}
