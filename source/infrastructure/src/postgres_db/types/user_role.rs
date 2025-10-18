use nimbus_auth_shared::types::UserRole;

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRoleDb {
    Default,
    Admin,
}

impl From<&UserRole> for UserRoleDb {
    fn from(value: &UserRole) -> Self {
        match value {
            UserRole::Default => Self::Default,
            UserRole::Admin => Self::Admin,
        }
    }
}

impl From<&UserRoleDb> for UserRole {
    fn from(value: &UserRoleDb) -> Self {
        match value {
            UserRoleDb::Default => Self::Default,
            UserRoleDb::Admin => Self::Admin,
        }
    }
}
