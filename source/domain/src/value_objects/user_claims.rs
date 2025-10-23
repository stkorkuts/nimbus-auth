use nimbus_auth_shared::types::UserRole;
use ulid::Ulid;

use crate::{
    entities::user::{User, value_objects::user_name::UserName},
    value_objects::identifier::Identifier,
};

#[derive(Debug, Clone)]
pub struct UserClaims {
    id: Identifier<Ulid, User>,
    name: UserName,
    role: UserRole,
}

impl UserClaims {
    pub fn new(id: Identifier<Ulid, User>, name: UserName, role: UserRole) -> Self {
        Self { id, name, role }
    }

    pub fn id(&self) -> &Identifier<Ulid, User> {
        &self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn role(&self) -> &UserRole {
        &self.role
    }
}
