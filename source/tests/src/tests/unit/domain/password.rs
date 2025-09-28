use nimbus_auth_domain::entities::user::value_objects::password::{
    Password, errors::PasswordError,
};
use zeroize::Zeroizing;

#[test]
fn valid_password() {
    let result = Password::from(&Zeroizing::new("StrongPassword123!".to_string()));
    assert!(result.is_ok())
}

#[test]
fn short_password() {
    let result = Password::from(&Zeroizing::new("Short".to_string()));
    assert!(matches!(result, Err(PasswordError::TooShort { .. })))
}

#[test]
fn long_password() {
    let long_password_str = "Qz7!aN4rP0@xVf9yU2$LwG3bH6%kT1mE5^jR8sC0ZqXnD7oF!gY4pJvK9uM2&BhQz7!aN4rP0@xVf9yU2$LwG3bH6%kT1mE5^jR8sC0ZqXnD7oF!gY4pJvK9uM2&Bh";
    let result = Password::from(&Zeroizing::new(long_password_str.to_string()));
    assert!(matches!(result, Err(PasswordError::TooLong { .. })))
}

#[test]
fn weak_password() {
    let result = Password::from(&Zeroizing::new("SimplePassword".to_string()));
    assert!(matches!(result, Err(PasswordError::TooWeak)))
}
