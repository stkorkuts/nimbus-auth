use nimbus_auth_domain::entities::user::value_objects::password::{
    Password, errors::PasswordError,
};
use nimbus_auth_shared::constants::{PASSWORD_MAX_LENGTH_INCLUSIVE, PASSWORD_MIN_LENGTH_INCLUSIVE};
use zeroize::Zeroizing;

#[test]
fn valid_password() {
    let result = Password::from(&Zeroizing::new("StrongPassword123!".to_string()));
    assert!(result.is_ok())
}

#[test]
fn short_password() {
    let short_password_str = "A".repeat(PASSWORD_MIN_LENGTH_INCLUSIVE - 1);
    let result = Password::from(&Zeroizing::new(short_password_str.to_string()));
    assert!(matches!(result, Err(PasswordError::TooShort { .. })))
}

#[test]
fn long_password() {
    let long_password_str = "A".repeat(PASSWORD_MAX_LENGTH_INCLUSIVE + 1);
    let result = Password::from(&Zeroizing::new(long_password_str.to_string()));
    assert!(matches!(result, Err(PasswordError::TooLong { .. })))
}

#[test]
fn weak_password() {
    let result = Password::from(&Zeroizing::new("SimplePassword".to_string()));
    assert!(matches!(result, Err(PasswordError::TooWeak)))
}
