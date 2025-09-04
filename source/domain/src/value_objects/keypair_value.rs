use crate::value_objects::keypair_value::errors::KeyPairValueError;

pub mod errors;

pub struct KeyPairValue {
    public_key_pem: Vec<u8>,
    private_key_pem: Vec<u8>,
}

impl KeyPairValue {
    pub fn new() -> Self {
        todo!()
    }

    pub fn restore(
        private_key_pem: Vec<u8>,
        public_key_pem: Vec<u8>,
    ) -> Result<Self, KeyPairValueError> {
        todo!()
    }

    pub fn private(&self) -> &[u8] {
        &self.private_key_pem
    }

    pub fn public(&self) -> &[u8] {
        &self.public_key_pem
    }
}
