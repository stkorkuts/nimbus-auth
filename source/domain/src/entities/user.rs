use ulid::Ulid;

use crate::{
    entities::{
        Entity,
        user::{
            specifications::{NewUserSpecification, RestoreUserSpecification},
            value_objects::{name::UserName, password_hash::PasswordHash},
        },
    },
    value_objects::identifier::Identifier,
};

pub mod errors;
pub mod specifications;
pub mod value_objects;

#[derive(Debug, Clone)]
pub struct User {
    id: Identifier<Ulid, User>,
    name: UserName,
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
        todo!()
    }

    pub fn restore(specs: RestoreUserSpecification) -> Self {
        todo!()
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }
}
