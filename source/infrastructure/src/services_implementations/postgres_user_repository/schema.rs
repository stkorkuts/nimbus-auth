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
use nimbus_auth_shared::types::UserRole;
use sqlx::prelude::FromRow;
use ulid::Ulid;

use crate::{
    postgres_db::types::user_role::UserRoleDb,
    services_implementations::postgres_user_repository::schema::errors::TryFromUserDbError,
};

pub mod errors;

#[derive(FromRow)]
pub struct GetUserDb {
    pub id: String,
    pub user_name: String,
    pub role: UserRoleDb,
    pub password_hash: String,
}

#[derive(FromRow)]
pub struct SaveUserDb {
    pub id: String,
    pub user_name: String,
    pub role: UserRoleDb,
    pub password_hash: String,
}

impl TryFrom<&GetUserDb> for User {
    type Error = TryFromUserDbError;

    fn try_from(value: &GetUserDb) -> Result<Self, Self::Error> {
        Ok(User::restore(RestoreUserSpecification {
            id: Identifier::from(Ulid::from_string(&value.id)?),
            user_name: UserName::from(&value.user_name)?,
            role: UserRole::from(&value.role),
            password_hash: PasswordHash::from(&value.password_hash)?,
        }))
    }
}

impl From<&User> for SaveUserDb {
    fn from(value: &User) -> Self {
        SaveUserDb {
            id: value.id().to_string(),
            user_name: value.name().to_string(),
            role: UserRoleDb::from(value.role()),
            password_hash: value.password_hash().to_string(),
        }
    }
}
