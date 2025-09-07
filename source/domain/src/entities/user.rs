use ulid::Ulid;

use crate::{
    entities::{
        Entity,
        user::{
            errors::NewUserError,
            specifications::{NewUserSpecification, RestoreUserSpecification},
            value_objects::name::UserName,
        },
    },
    value_objects::identifier::Identifier,
};

pub mod errors;
pub mod specifications;
pub mod value_objects;

pub struct User {
    id: Identifier<Ulid, User>,
    name: UserName,
}

impl Entity<Ulid> for User {
    type Id = Identifier<Ulid, User>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl User {
    pub fn new(specs: NewUserSpecification) -> Result<Self, NewUserError> {
        todo!()
    }

    pub fn restore(specs: RestoreUserSpecification) -> Result<Self, NewUserError> {
        todo!()
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }
}
