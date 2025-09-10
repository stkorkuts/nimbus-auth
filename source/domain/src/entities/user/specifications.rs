use crate::entities::user::value_objects::{name::UserName, password::Password};

pub struct NewUserSpecification {
    pub user_name: UserName,
    pub password: Password,
}

pub struct RestoreUserSpecification {
    pub user_name: UserName,
    pub password: Password,
}
