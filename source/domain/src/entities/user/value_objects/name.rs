use std::ops::Deref;

use crate::entities::user::value_objects::name::errors::UserNameError;

pub mod errors;

pub struct UserName {
    value: String,
}

impl UserName {
    pub fn from(value: &str) -> Result<Self, UserNameError> {
        todo!()
    }
}

impl Deref for UserName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
