use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyPairValueError {
    #[error("invalid private key format")]
    InvalidPrivateKeyFormat,
    #[error("invalid public key format")]
    InvalidPublicKeyFormat,
    #[error("keys do not match each other")]
    KeysDoNotMatch,
}
