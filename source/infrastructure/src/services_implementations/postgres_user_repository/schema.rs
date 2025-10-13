use nimbus_auth_application::services::user_repository::errors::UserRepositoryError;
use nimbus_auth_domain::{
    entities::{
        Entity,
        user::{
            User,
            specifications::RestoreUserSpecification,
            value_objects::{password_hash::PasswordHash, user_name::UserName},
        },
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::errors::ErrorBoxed;
use sqlx::prelude::FromRow;
use ulid::Ulid;

#[derive(FromRow)]
pub struct GetUserDb {
    pub id: String,
    pub user_name: String,
    pub password_hash: String,
}

#[derive(FromRow)]
pub struct SaveUserDb {
    pub id: String,
    pub user_name: String,
    pub password_hash: String,
}

impl GetUserDb {
    pub fn into_domain(self) -> Result<User, UserRepositoryError> {
        Ok(User::restore(RestoreUserSpecification {
            id: Identifier::from(Ulid::from_string(&self.id).map_err(ErrorBoxed::from)?),
            user_name: UserName::from(&self.user_name)?,
            password_hash: PasswordHash::from(&self.password_hash)?,
        }))
    }
}

impl From<&User> for SaveUserDb {
    fn from(value: &User) -> Self {
        SaveUserDb {
            id: value.id().to_string(),
            user_name: value.name().to_string(),
            password_hash: value.password_hash().to_string(),
        }
    }
}
