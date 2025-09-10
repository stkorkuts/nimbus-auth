use std::ops::Deref;

use crate::entities::user::value_objects::password::errors::PasswordError;

pub mod errors;

pub struct Password {
    value: String,
}

impl Password {
    pub fn from(value: &str) -> Result<Self, PasswordError> {
        todo!()
    }
}

impl Deref for Password {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
