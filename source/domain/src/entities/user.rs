use nimbus_auth_shared::types::UserRole;
use ulid::Ulid;

use crate::{
    entities::{
        Entity,
        user::{
            specifications::{NewUserSpecification, RestoreUserSpecification},
            value_objects::{password_hash::PasswordHash, user_name::UserName},
        },
    },
    value_objects::identifier::{Identifier, IdentifierOfType},
};

pub mod errors;
pub mod specifications;
pub mod value_objects;

#[derive(Debug, Clone)]
pub struct User {
    id: Identifier<Ulid, User>,
    name: UserName,
    role: UserRole,
    password_hash: PasswordHash,
}

impl Entity<Ulid> for User {
    type Id = Identifier<Ulid, User>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl User {
    pub fn new(specs: NewUserSpecification) -> Self {
        Self {
            id: Identifier::new(),
            name: specs.user_name,
            password_hash: specs.password_hash,
            role: UserRole::Default,
        }
    }

    pub fn restore(specs: RestoreUserSpecification) -> Self {
        Self {
            id: specs.id,
            name: specs.user_name,
            password_hash: specs.password_hash,
            role: specs.role,
        }
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn role(&self) -> &UserRole {
        &self.role
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn set_role(&mut self, role: UserRole) {
        self.role = role;
    }
}
