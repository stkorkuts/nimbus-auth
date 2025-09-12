use ed25519_dalek::pkcs8::{self};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyPairValueError {
    #[error(transparent)]
    Pkcs8Error(#[from] pkcs8::Error),
    #[error(transparent)]
    Pkcs8SpkiError(#[from] pkcs8::spki::Error),
    #[error("invalid private key format")]
    InvalidPrivateKeyFormat,
    #[error("invalid public key format")]
    InvalidPublicKeyFormat,
    #[error("keys do not match each other")]
    KeysDoNotMatch,
}
