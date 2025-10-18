use nimbus_auth_domain::entities::{Entity, user::User};
use nimbus_auth_shared::types::UserRole;

pub struct UserDto {
    pub id: String,
    pub name: String,
    pub role: UserRole,
}

impl From<&User> for UserDto {
    fn from(value: &User) -> Self {
        Self {
            id: value.id().to_string(),
            name: value.name().to_string(),
            role: value.role().clone(),
        }
    }
}
