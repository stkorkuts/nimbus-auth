use std::ops::Deref;

use crate::entities::user::value_objects::name::errors::UserNameError;

pub mod errors;

#[derive(Clone)]
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

impl ToString for UserName {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
