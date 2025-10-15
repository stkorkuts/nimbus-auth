use std::fmt::Display;

use nimbus_auth_shared::constants::{USERNAME_MAX_LENGTH_INCLUSIVE, USERNAME_MIN_LENGTH_INCLUSIVE};

use crate::entities::user::value_objects::user_name::errors::UserNameError;

pub mod errors;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserName {
    value: String,
}

impl UserName {
    pub fn from(value: &str) -> Result<Self, UserNameError> {
        Self::validate(value)?;
        Ok(Self {
            value: value.to_string(),
        })
    }

    fn validate(value: &str) -> Result<(), UserNameError> {
        let length = value.len();
        if length < USERNAME_MIN_LENGTH_INCLUSIVE {
            return Err(UserNameError::TooShort {
                min_length: USERNAME_MIN_LENGTH_INCLUSIVE,
            });
        }
        if length > USERNAME_MAX_LENGTH_INCLUSIVE {
            return Err(UserNameError::TooLong {
                max_length: USERNAME_MAX_LENGTH_INCLUSIVE,
            });
        }
        match value.chars().all(|ch| ch.is_ascii_alphanumeric()) {
            true => Ok(()),
            false => Err(UserNameError::InvalidCharacters),
        }
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
