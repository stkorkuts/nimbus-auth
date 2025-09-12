use ed25519_dalek::{
    SigningKey,
    pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey, spki::der::pem::LineEnding},
};

use crate::entities::keypair::value_objects::errors::KeyPairValueError;

pub mod errors;

pub struct KeyPairValue {
    signing_key: SigningKey,
}

impl KeyPairValue {
    pub fn from(private_key_pem: &str) -> Result<Self, KeyPairValueError> {
        let signing_key = SigningKey::from_pkcs8_pem(private_key_pem)
            .map_err(|_| KeyPairValueError::InvalidPrivateKeyFormat)?;
        Ok(Self { signing_key })
    }

    pub fn private_key_pem(&self) -> Vec<u8> {
        self.signing_key
            .to_pkcs8_pem(LineEnding::LF)
            .unwrap()
            .to_string()
            .into_bytes()
    }

    pub fn public_key_pem(&self) -> Vec<u8> {
        let verifying_key = self.signing_key.verifying_key();
        verifying_key
            .to_public_key_pem(LineEnding::LF)
            .unwrap()
            .into_bytes()
    }
}
