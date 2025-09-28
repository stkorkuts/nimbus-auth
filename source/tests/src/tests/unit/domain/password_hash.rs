use argon2::password_hash::{SaltString, rand_core::OsRng};
use nimbus_auth_domain::entities::user::value_objects::{
    password::Password,
    password_hash::{PasswordHash, errors::PasswordHashError},
};
use zeroize::Zeroizing;

const VALID_PASSWORD: &str = "StrongPassword123!";

#[test]
fn same_password_valid_salt() {
    let salt = SaltString::generate(&mut OsRng);
    let password_to_hash = Password::from(&Zeroizing::new(VALID_PASSWORD.to_string())).unwrap();
    let hash = PasswordHash::hash(password_to_hash, salt.as_str()).unwrap();
    let password_to_verify = Password::from(&Zeroizing::new(VALID_PASSWORD.to_string())).unwrap();
    assert!(hash.verify(&password_to_verify))
}

#[test]
fn invalid_salt() {
    let invalid_salt = "invalid_salt";
    let password_to_hash = Password::from(&Zeroizing::new(VALID_PASSWORD.to_string())).unwrap();
    let result = PasswordHash::hash(password_to_hash, invalid_salt);
    assert!(matches!(result, Err(PasswordHashError::Salt)))
}

#[test]
fn invalid_password() {
    let invalid_password = "WrongPassword123!";
    let salt = SaltString::generate(&mut OsRng);
    let password_to_hash = Password::from(&Zeroizing::new(VALID_PASSWORD.to_string())).unwrap();
    let hash = PasswordHash::hash(password_to_hash, salt.as_str()).unwrap();
    let password_to_verify = Password::from(&Zeroizing::new(invalid_password.to_string())).unwrap();
    assert!(!hash.verify(&password_to_verify))
}
