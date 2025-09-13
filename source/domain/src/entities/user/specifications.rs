use ulid::Ulid;

use crate::{
    entities::user::{
        User,
        value_objects::{name::UserName, password::Password, password_hash::PasswordHash},
    },
    value_objects::identifier::Identifier,
};

pub struct NewUserSpecification {
    pub user_name: UserName,
    pub password: Password,
}

pub struct RestoreUserSpecification {
    pub id: Identifier<Ulid, User>,
    pub user_name: UserName,
    pub password_hash: PasswordHash,
}
