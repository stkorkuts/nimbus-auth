use ulid::Ulid;

pub struct GetPublicKeyRequest {
    pub key_id: Ulid,
}

pub struct GetPublicKeyResponse {
    pub public_key_pem: Vec<u8>,
}
