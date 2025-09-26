use ed25519_dalek::{
    SigningKey,
    pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey, spki::der::pem::LineEnding},
};
use zeroize::Zeroizing;

use crate::entities::keypair::value_objects::errors::KeyPairValueError;

pub mod errors;

#[derive(Debug, Clone)]
pub struct KeyPairValue {
    signing_key: Zeroizing<[u8; 32]>,
}

impl KeyPairValue {
    pub fn from_pem(private_key_pem: Zeroizing<String>) -> Result<Self, KeyPairValueError> {
        let signing_key = SigningKey::from_pkcs8_pem(private_key_pem.as_str())
            .map_err(|_| KeyPairValueError::InvalidPrivateKeyFormat)?;
        Ok(Self {
            signing_key: Zeroizing::new(signing_key.to_bytes()),
        })
    }

    pub fn private_key_pem(&self) -> Zeroizing<String> {
        SigningKey::from_bytes(&self.signing_key)
            .to_pkcs8_pem(LineEnding::LF)
            .unwrap()
    }

    pub fn public_key_pem(&self) -> String {
        let verifying_key = SigningKey::from_bytes(&self.signing_key).verifying_key();
        verifying_key.to_public_key_pem(LineEnding::LF).unwrap()
    }
}
