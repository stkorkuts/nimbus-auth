use std::path::PathBuf;

use nimbus_auth_application::services::keypair_repository::{
    KeyPairRepository, KeyPairRepositoryWithTransaction, errors::KeyPairRepositoryError,
};
use nimbus_auth_domain::{
    entities::keypair::{Active, KeyPair, SomeKeyPair},
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::StaticPinnedFuture;
use ulid::Ulid;

pub struct FileSystemKeyPairRepository {
    keypairs_location: PathBuf,
}

pub struct FileSystemKeyPairRepositoryWithTransaction {
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
        id: &Identifier<Ulid, SomeKeyPair<'static>>,
    ) -> StaticPinnedFuture<Option<SomeKeyPair<'static>>, KeyPairRepositoryError> {
        todo!()
    }

    fn get_active(&self) -> StaticPinnedFuture<Option<KeyPair<Active>>, KeyPairRepositoryError> {
        todo!()
    }

    fn save(&self, keypair: SomeKeyPair) -> StaticPinnedFuture<(), KeyPairRepositoryError> {
        todo!()
    }
}

impl KeyPairRepositoryWithTransaction for FileSystemKeyPairRepositoryWithTransaction {
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), KeyPairRepositoryError> {
        todo!()
    }

    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), KeyPairRepositoryError> {
        todo!()
    }

    fn get_by_id(
        self: Box<Self>,
        id: &Identifier<Ulid, SomeKeyPair<'static>>,
    ) -> StaticPinnedFuture<
        (
            Box<dyn KeyPairRepositoryWithTransaction>,
            Option<SomeKeyPair<'static>>,
        ),
        KeyPairRepositoryError,
    > {
        todo!()
    }

    fn get_active(
        self: Box<Self>,
    ) -> StaticPinnedFuture<
        (
            Box<dyn KeyPairRepositoryWithTransaction>,
            Option<KeyPair<Active>>,
        ),
        KeyPairRepositoryError,
    > {
        todo!()
    }

    fn save(
        self: Box<Self>,
        keypair: SomeKeyPair,
    ) -> StaticPinnedFuture<(Box<dyn KeyPairRepositoryWithTransaction>, ()), KeyPairRepositoryError>
    {
        todo!()
    }
}
