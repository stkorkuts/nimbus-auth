use nimbus_auth_domain::value_objects::user_claims::UserClaims;
use nimbus_auth_shared::types::UserRole;

pub struct UserClaimsDto {
    pub id: String,
    pub name: String,
    pub role: UserRole,
}

impl From<&UserClaims> for UserClaimsDto {
    fn from(value: &UserClaims) -> Self {
        Self {
            id: value.id().to_string(),
            name: value.name().to_string(),
            role: value.role().clone(),
        }
    }
}
