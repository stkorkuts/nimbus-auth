use crate::entities::user::value_objects::{
    password::Password, password_hash::errors::PasswordHashError,
};

pub mod errors;

#[derive(Clone)]
pub struct PasswordHash {}

impl PasswordHash {
    pub fn from(password: Password) -> Result<Self, PasswordHashError> {
        todo!()
    }

    pub fn verify(&self, password: Password) -> bool {
        todo!()
    }
}
