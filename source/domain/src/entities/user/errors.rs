use thiserror::Error;

use crate::entities::user::value_objects::name::errors::UserNameError;

#[derive(Debug, Error)]
pub enum UserError {}
