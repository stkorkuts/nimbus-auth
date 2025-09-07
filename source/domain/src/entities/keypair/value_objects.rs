use ed25519_dalek::{
    SigningKey, VerifyingKey,
    pkcs8::{
        DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey,
        spki::der::pem::LineEnding,
    },
};
use rand::rngs::OsRng;

use crate::entities::keypair::value_objects::errors::KeyPairValueError;

pub mod errors;

pub struct KeyPairValue {
    public_key_pem: Vec<u8>,
    private_key_pem: Vec<u8>,
}

impl KeyPairValue {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        let verifying_key = signing_key.verifying_key();

        let private_pem = signing_key
            .to_pkcs8_pem(LineEnding::LF)
            .unwrap()
            .to_string()
            .into_bytes();

        let public_pem = verifying_key
            .to_public_key_pem(LineEnding::LF)
            .unwrap()
            .into_bytes();

        Self {
            public_key_pem: public_pem,
            private_key_pem: private_pem,
        }
    }

    pub fn restore(private_key_pem: &str, public_key_pem: &str) -> Result<Self, KeyPairValueError> {
        let signing_key = SigningKey::from_pkcs8_pem(private_key_pem)
            .map_err(|_| KeyPairValueError::InvalidPrivateKeyFormat)?;

        let verifying_key = signing_key.verifying_key();

        let provided_verifying_key = VerifyingKey::from_public_key_pem(public_key_pem)
            .map_err(|_| KeyPairValueError::InvalidPublicKeyFormat)?;

        if verifying_key != provided_verifying_key {
            return Err(KeyPairValueError::KeysDoNotMatch);
        }

        Ok(Self {
            private_key_pem: private_key_pem.as_bytes().to_vec(),
            public_key_pem: public_key_pem.as_bytes().to_vec(),
        })
    }

    pub fn private(&self) -> &[u8] {
        &self.private_key_pem
    }

    pub fn public(&self) -> &[u8] {
        &self.public_key_pem
    }
}
