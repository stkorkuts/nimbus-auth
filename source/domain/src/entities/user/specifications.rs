use ulid::Ulid;

use crate::{
    entities::user::{
        User,
        value_objects::{password_hash::PasswordHash, user_name::UserName},
    },
    value_objects::identifier::Identifier,
};

pub struct NewUserSpecification {
    pub user_name: UserName,
    pub password_hash: PasswordHash,
}

pub struct RestoreUserSpecification {
    pub id: Identifier<Ulid, User>,
    pub user_name: UserName,
    pub password_hash: PasswordHash,
}
