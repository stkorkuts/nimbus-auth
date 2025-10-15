use nimbus_auth_shared::constants::{PASSWORD_MAX_LENGTH_INCLUSIVE, PASSWORD_MIN_LENGTH_INCLUSIVE};
use zeroize::Zeroizing;

use crate::entities::user::value_objects::password::errors::PasswordError;

pub mod errors;
#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct Password {
    value: Zeroizing<String>,
}

struct PasswordCharsValidation {
    has_ascii_uppercase: bool,
    has_ascii_lowercase: bool,
    has_ascii_digit: bool,
    has_ascii_punctuation: bool,
    has_non_ascii: bool,
    has_spaces: bool,
}

impl PasswordCharsValidation {
    pub fn new() -> Self {
        Self {
            has_ascii_uppercase: false,
            has_ascii_lowercase: false,
            has_ascii_digit: false,
            has_ascii_punctuation: false,
            has_non_ascii: false,
            has_spaces: false,
        }
    }

    pub fn validate_next_char(mut self, ch: char) -> Self {
        self.has_ascii_uppercase = self.has_ascii_uppercase || ch.is_ascii_uppercase();
        self.has_ascii_lowercase = self.has_ascii_lowercase || ch.is_ascii_lowercase();
        self.has_ascii_digit = self.has_ascii_digit || ch.is_ascii_digit();
        self.has_ascii_punctuation = self.has_ascii_punctuation || ch.is_ascii_punctuation();
        self.has_spaces = self.has_spaces || ch.is_whitespace();
        self.has_non_ascii = self.has_non_ascii || !ch.is_ascii();
        self
    }

    pub fn validate(self) -> Result<(), PasswordError> {
        if !(self.has_ascii_uppercase
            && self.has_ascii_lowercase
            && self.has_ascii_digit
            && self.has_ascii_punctuation)
        {
            return Err(PasswordError::TooWeak);
        }

        if self.has_non_ascii || self.has_spaces {
            return Err(PasswordError::InvalidCharacters);
        }

        Ok(())
    }
}

impl Password {
    pub fn from(value: &Zeroizing<String>) -> Result<Self, PasswordError> {
        Self::validate(value)?;
        Ok(Self {
            value: value.clone(),
        })
    }

    fn validate(value: &str) -> Result<(), PasswordError> {
        let length = value.len();
        if length < PASSWORD_MIN_LENGTH_INCLUSIVE {
            return Err(PasswordError::TooShort {
                min_length: PASSWORD_MIN_LENGTH_INCLUSIVE,
            });
        }
        if length > PASSWORD_MAX_LENGTH_INCLUSIVE {
            return Err(PasswordError::TooLong {
                max_length: PASSWORD_MAX_LENGTH_INCLUSIVE,
            });
        }
        value
            .chars()
            .fold(PasswordCharsValidation::new(), |checks, ch| {
                checks.validate_next_char(ch)
            })
            .validate()
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
