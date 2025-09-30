use nimbus_auth_domain::entities::user::value_objects::user_name::{
    UserName, errors::UserNameError,
};
use nimbus_auth_shared::constants::{USERNAME_MAX_LENGTH_INCLUSIVE, USERNAME_MIN_LENGTH_INCLUSIVE};

#[test]
fn valid_user_name() {
    let result = UserName::from("stanislau123");
    assert!(result.is_ok())
}

#[test]
fn short_user_name() {
    let result = UserName::from(&"A".repeat(USERNAME_MIN_LENGTH_INCLUSIVE - 1));
    assert!(matches!(
        result,
        Err(UserNameError::TooShort {
            min_length: USERNAME_MIN_LENGTH_INCLUSIVE
        })
    ))
}

#[test]
fn long_user_name() {
    let result = UserName::from(&"A".repeat(USERNAME_MAX_LENGTH_INCLUSIVE + 1));
    assert!(matches!(
        result,
        Err(UserNameError::TooLong {
            max_length: USERNAME_MAX_LENGTH_INCLUSIVE
        })
    ))
}

#[test]
fn invalid_user_name() {
    let result = UserName::from("Stanislau 123");
    assert!(matches!(result, Err(UserNameError::InvalidCharacters)))
}
