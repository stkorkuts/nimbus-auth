use nimbus_auth_domain::{
    entities::{
        Entity,
        user::{
            User,
            errors::UserError,
            specifications::RestoreUserSpecification,
            value_objects::{name::UserName, password_hash::PasswordHash},
        },
    },
    value_objects::identifier::{Identifier, IdentifierOfType},
};
use nimbus_auth_shared::errors::ErrorBoxed;
use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct TestUser {
    pub id: Ulid,
    pub user_name: String,
    pub password_hash: String,
}

impl From<&User> for TestUser {
    fn from(value: &User) -> Self {
        TestUser {
            id: *value.id().value(),
            user_name: value.name().to_string(),
            password_hash: value.password_hash().to_string(),
        }
    }
}

impl TestUser {
    pub fn into_domain(self) -> Result<User, ErrorBoxed> {
        Ok(User::restore(RestoreUserSpecification {
            id: Identifier::from(self.id),
            user_name: UserName::from(&self.user_name)?,
            password_hash: PasswordHash::from(&self.password_hash)?,
        }))
    }
}
