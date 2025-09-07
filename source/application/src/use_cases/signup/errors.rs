use nimbus_auth_domain::entities::user::value_objects::name::errors::UserNameError;
use thiserror::Error;

use crate::services::user_repository::errors::UserRepositoryError;

#[derive(Debug, Error)]
pub enum SignUpError {
    #[error(transparent)]
    UserRepository(#[from] UserRepositoryError),
    #[error(transparent)]
    InvalidUserName(#[from] UserNameError),
    #[error("user with name: {user_name} already exists")]
    UserAlreadyExists { user_name: String },
}
