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
