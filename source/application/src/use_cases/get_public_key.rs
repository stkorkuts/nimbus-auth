use std::path::PathBuf;

use crate::use_cases::{GetPublicKeyError, GetPublicKeyRequest, GetPublicKeyResponse};

pub mod errors;
pub mod schema;

pub async fn handle_get_public_key(
    GetPublicKeyRequest {}: GetPublicKeyRequest,
    public_key_path: &PathBuf,
) -> Result<GetPublicKeyResponse, GetPublicKeyError> {
    todo!()
}
