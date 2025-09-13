use std::ops::Deref;

use crate::entities::user::value_objects::password::errors::PasswordError;

pub mod errors;

#[derive(Clone)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn from(value: &str) -> Result<Self, PasswordError> {
        todo!()
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ToString for Password {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
