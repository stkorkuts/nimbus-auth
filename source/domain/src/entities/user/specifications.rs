use crate::entities::user::value_objects::{
    name::UserName, password::Password, password_hash::PasswordHash,
};

pub struct NewUserSpecification {
    pub user_name: UserName,
    pub password: Password,
}

pub struct RestoreUserSpecification {
    pub user_name: UserName,
    pub password: PasswordHash,
}
