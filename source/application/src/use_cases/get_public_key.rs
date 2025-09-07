use std::sync::Arc;

use crate::{
    services::keypair_repository::KeyPairRepository,
    use_cases::{GetPublicKeyError, GetPublicKeyRequest, GetPublicKeyResponse},
};

pub mod errors;
pub mod schema;

pub async fn handle_get_public_key(
    GetPublicKeyRequest { key_id: _key_id }: GetPublicKeyRequest,
    keypair_repository: Arc<dyn KeyPairRepository>,
) -> Result<GetPublicKeyResponse, GetPublicKeyError> {
    Ok(GetPublicKeyResponse {
        public_key_pem: keypair_repository
            .get_active(None)
            .await?
            .ok_or(GetPublicKeyError::ActiveKeyPairNotFound)?
            .value()
            .public()
            .to_vec(),
    })
}
