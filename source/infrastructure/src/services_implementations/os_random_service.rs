use argon2::password_hash::SaltString;
use ed25519_dalek::{
    SigningKey,
    pkcs8::{EncodePrivateKey, spki::der::pem::LineEnding},
};
use nimbus_auth_application::services::random_service::{
    RandomService, errors::RandomServiceError,
};
use nimbus_auth_shared::futures::{StaticPinnedFuture, pin_static_future};
use rand::rngs::OsRng;
use zeroize::Zeroizing;

pub struct OsRandomService {}

impl OsRandomService {
    pub fn new() -> Self {
        OsRandomService {}
    }
}

impl RandomService for OsRandomService {
    fn get_random_private_key_pem(
        &self,
    ) -> StaticPinnedFuture<Zeroizing<String>, RandomServiceError> {
        pin_static_future(async {
            let mut rng = OsRng;
            let signing_key = SigningKey::generate(&mut rng);
            Ok(signing_key.to_pkcs8_pem(LineEnding::LF).unwrap())
        })
    }

    fn get_random_salt_b64(&self) -> StaticPinnedFuture<String, RandomServiceError> {
        pin_static_future(async { Ok(SaltString::generate(&mut OsRng).as_str().to_string()) })
    }
}
