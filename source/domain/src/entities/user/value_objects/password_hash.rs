use std::fmt::Display;

use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString},
};

use crate::entities::user::value_objects::{
    password::Password, password_hash::errors::PasswordHashError,
};

pub mod errors;

#[derive(Clone, Debug)]
pub struct PasswordHash {
    value: String,
}

impl PasswordHash {
    pub fn hash(password: Password, salt_b64: &str) -> Result<Self, PasswordHashError> {
        let salt = SaltString::from_b64(salt_b64).map_err(|_| PasswordHashError::Salt)?;
        let hash = Argon2::default()
            .hash_password(password.value().as_bytes(), &salt)
            .map_err(|err| PasswordHashError::Hash(err))?;
        Ok(Self {
            value: hash.to_string(),
        })
    }

    pub fn from(value: &str) -> Result<Self, PasswordHashError> {
        let hash = argon2::password_hash::PasswordHash::new(value)
            .map_err(|err| PasswordHashError::Hash(err))?;
        Ok(Self {
            value: hash.to_string(),
        })
    }

    pub fn verify(&self, password: &Password) -> bool {
        let hash = argon2::password_hash::PasswordHash::new(&self.value).unwrap();
        Argon2::default()
            .verify_password(password.value().as_bytes(), &hash)
            .is_ok()
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Display for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Password hash: {}", self.value)
    }
}
