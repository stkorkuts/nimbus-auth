use std::path::PathBuf;

use nimbus_auth_application::services::keypair_repository::{
    KeyPairRepository, KeyPairRepositoryWithTransaction, errors::KeyPairRepositoryError,
};
use nimbus_auth_domain::{
    entities::keypair::{
        Active, InitializedKeyPair, InitializedKeyPairRef, KeyPair, Uninitialized,
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::StaticPinnedFuture;
use ulid::Ulid;

pub struct FileSystemKeyPairRepository {
    keypairs_location: PathBuf,
}

impl FileSystemKeyPairRepository {
    pub async fn init(keypairs_location: &PathBuf) -> Result<Self, KeyPairRepositoryError> {
        todo!();
        Ok(Self {
            keypairs_location: keypairs_location.clone(),
        })
    }
}

impl KeyPairRepository for FileSystemKeyPairRepository {
    fn start_transaction(
        &self,
    ) -> StaticPinnedFuture<Box<dyn KeyPairRepositoryWithTransaction>, KeyPairRepositoryError> {
        todo!()
    }

    fn get_by_id(
        &self,
        id: &Identifier<Ulid, KeyPair<Uninitialized>>,
    ) -> StaticPinnedFuture<Option<InitializedKeyPair>, KeyPairRepositoryError> {
        todo!()
    }

    fn get_active(&self) -> StaticPinnedFuture<Option<KeyPair<Active>>, KeyPairRepositoryError> {
        todo!()
    }

    fn save(
        &self,
        keypair: InitializedKeyPairRef,
    ) -> StaticPinnedFuture<(), KeyPairRepositoryError> {
        todo!()
    }
}
