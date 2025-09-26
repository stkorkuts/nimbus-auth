use zeroize::Zeroizing;

use crate::entities::user::value_objects::password::errors::PasswordError;

pub mod errors;

#[derive(Clone)]
pub struct Password {
    value: Zeroizing<String>,
}

impl Password {
    pub fn from(value: &Zeroizing<String>) -> Result<Self, PasswordError> {
        Self::validate(value)?;
        Ok(Self {
            value: value.clone(),
        })
    }

    fn validate(value: &str) -> Result<(), PasswordError> {
        todo!()
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
