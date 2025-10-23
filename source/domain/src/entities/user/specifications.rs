use crate::{
    entities::user::value_objects::{password_hash::PasswordHash, user_name::UserName},
    value_objects::user_claims::UserClaims,
};

pub struct NewUserSpecification {
    pub user_name: UserName,
    pub password_hash: PasswordHash,
}

pub struct RestoreUserSpecification {
    pub claims: UserClaims,
    pub password_hash: PasswordHash,
}
