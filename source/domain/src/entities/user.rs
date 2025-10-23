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
    value_objects::{
        identifier::{Identifier, IdentifierOfType},
        user_claims::UserClaims,
    },
};

pub mod errors;
pub mod specifications;
pub mod value_objects;

#[derive(Debug, Clone)]
pub struct User {
    claims: UserClaims,
    password_hash: PasswordHash,
}

impl Entity<Ulid> for User {
    type Id = Identifier<Ulid, User>;

    fn id(&self) -> &Self::Id {
        self.claims.id()
    }
}

impl User {
    pub fn new(specs: NewUserSpecification) -> Self {
        Self {
            claims: UserClaims::new(Identifier::new(), specs.user_name, UserRole::Default),
            password_hash: specs.password_hash,
        }
    }

    pub fn restore(specs: RestoreUserSpecification) -> Self {
        Self {
            claims: specs.claims,
            password_hash: specs.password_hash,
        }
    }

    pub fn name(&self) -> &UserName {
        self.claims.name()
    }

    pub fn role(&self) -> &UserRole {
        self.claims.role()
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn claims(&self) -> &UserClaims {
        &self.claims
    }

    pub fn with_new_role(self, role: UserRole) -> Self {
        Self::restore(RestoreUserSpecification {
            claims: UserClaims::new(self.claims.id().clone(), self.claims.name().clone(), role),
            password_hash: self.password_hash,
        })
    }
}
