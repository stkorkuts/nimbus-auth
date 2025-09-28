use argon2::password_hash;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordHashError {
    #[error("invalid salt provided")]
    Salt,
    #[error("error while hashing")]
    Hash(password_hash::Error),
}
