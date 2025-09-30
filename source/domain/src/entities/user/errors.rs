use thiserror::Error;

use crate::entities::user::value_objects::user_name::errors::UserNameError;

#[derive(Debug, Error)]
pub enum UserError {}
