use crate::entities::user::value_objects::{
    e2e_key_hash::E2eKeyHash, encrypted_master_key::EncryptedMasterKey, name::UserName,
    password::Password,
};

pub struct NewUserSpecification {
    pub user_name: UserName,
    pub password: Password,
    pub e2e_key_hash: E2eKeyHash,
    pub encrypted_master_key: EncryptedMasterKey,
}

pub struct RestoreUserSpecification {
    pub user_name: UserName,
    pub password: Password,
    pub e2e_key_hash: E2eKeyHash,
    pub encrypted_master_key: EncryptedMasterKey,
}
