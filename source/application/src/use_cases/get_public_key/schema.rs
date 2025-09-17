pub struct GetPublicKeyRequest<'a> {
    pub key_id: Option<&'a str>,
}

pub struct GetPublicKeyResponse {
    pub public_key_pem: Vec<u8>,
}
