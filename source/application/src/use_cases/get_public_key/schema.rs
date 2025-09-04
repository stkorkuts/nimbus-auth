use ulid::Ulid;

pub struct GetPublicKeyRequest {
    pub key_id: Ulid,
}

pub struct GetPublicKeyResponse {}
