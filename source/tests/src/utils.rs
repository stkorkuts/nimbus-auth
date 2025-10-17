use argon2::password_hash::rand_core::OsRng;
use ed25519_dalek::{SigningKey, pkcs8::EncodePrivateKey, pkcs8::spki::der::pem::LineEnding};
use nimbus_auth_domain::entities::keypair::{
    Active, KeyPair, SomeKeyPair, specifications::NewKeyPairSpecification,
    value_objects::KeyPairValue,
};

pub fn get_active_keypair() -> KeyPair<Active> {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let pem = signing_key.to_pkcs8_pem(LineEnding::LF).unwrap();
    SomeKeyPair::new(NewKeyPairSpecification {
        value: KeyPairValue::from_pem(pem).expect("key pair value should have been constructed"),
    })
}
