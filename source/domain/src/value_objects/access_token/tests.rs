use argon2::password_hash::SaltString;
use ed25519_dalek::{SigningKey, pkcs8::EncodePrivateKey, pkcs8::spki::der::pem::LineEnding};
use nimbus_auth_shared::{
    constants::ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT, types::AccessTokenExpirationSeconds,
};
use rand::rngs::OsRng;
use time::OffsetDateTime;
use zeroize::Zeroizing;

use crate::{
    entities::{
        Entity,
        keypair::{
            Active, KeyPair, SomeKeyPair, specifications::NewKeyPairSpecification,
            value_objects::KeyPairValue,
        },
        user::{
            User,
            specifications::NewUserSpecification,
            value_objects::{password::Password, password_hash::PasswordHash, user_name::UserName},
        },
    },
    value_objects::access_token::AccessToken,
};

const VALID_USER_NAME: &str = "validuser123";
const VALID_PASSWORD: &str = "StrongPassword123!";
const PRIVATE_KEY_PEM: &str = "-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIMUBs5zfkuEGgSLwrUo2vln82Z8hUySsoI+dyA3AonDV
-----END PRIVATE KEY-----
";

fn get_user() -> User {
    let user_name = UserName::from(VALID_USER_NAME)
        .expect("username should have been constructed successfully");
    let password = Password::from(&Zeroizing::new(VALID_PASSWORD.to_string()))
        .expect("password should have been constructed successfully");
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = PasswordHash::hash(password, salt.as_str())
        .expect("password should have been cached successfully");

    User::new(NewUserSpecification {
        user_name: user_name,
        password_hash: password_hash,
    })
}

fn get_keypair() -> KeyPair<Active> {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let pem = signing_key.to_pkcs8_pem(LineEnding::LF).unwrap();
    SomeKeyPair::new(NewKeyPairSpecification {
        value: KeyPairValue::from_pem(pem).expect("key pair value should have been constructed"),
    })
}

#[test]
fn encode_decode() {
    let user = get_user();
    let keypair = get_keypair();

    let access_token = AccessToken::new(
        user.id().clone(),
        OffsetDateTime::now_utc(),
        AccessTokenExpirationSeconds(ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT),
    );
    let signed_token = access_token
        .sign(&keypair)
        .expect("token should have been signed successfully");

    let result = AccessToken::verify_with_active(&signed_token, keypair);
    assert!(matches!(result, Ok(..)));
}
