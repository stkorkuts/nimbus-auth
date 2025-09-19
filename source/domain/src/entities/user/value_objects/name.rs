use std::fmt::Display;

use crate::entities::user::value_objects::name::errors::UserNameError;

pub mod errors;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserName {
    value: String,
}

impl UserName {
    pub fn from(value: &str) -> Result<Self, UserNameError> {
        todo!()
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User name: {}", self.value)
    }
}
