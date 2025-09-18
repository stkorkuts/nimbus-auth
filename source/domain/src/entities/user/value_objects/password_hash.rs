use crate::entities::user::value_objects::{
    password::Password, password_hash::errors::PasswordHashError,
};

pub mod errors;

#[derive(Clone, Debug)]
pub struct PasswordHash {
    value: String,
}

impl PasswordHash {
    pub fn hash(password: Password) -> Result<Self, PasswordHashError> {
        todo!()
    }

    pub fn from(value: &str) -> Result<Self, PasswordHashError> {
        todo!()
    }

    pub fn verify(&self, password: &Password) -> bool {
        todo!()
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ToString for PasswordHash {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
