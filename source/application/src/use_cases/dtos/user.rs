use nimbus_auth_domain::entities::{Entity, user::User};

pub struct UserDto {
    pub id: String,
    pub name: String,
}

impl From<&User> for UserDto {
    fn from(value: &User) -> Self {
        Self {
            id: value.id().to_string(),
            name: value.name().to_string(),
        }
    }
}
