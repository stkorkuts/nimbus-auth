use std::sync::Arc;

use dashmap::DashMap;
use nimbus_auth_application::services::keypair_repository::{
    KeyPairRepository, KeyPairRepositoryWithTransaction, errors::KeyPairRepositoryError,
};
use nimbus_auth_domain::{
    entities::keypair::{Active, KeyPair, SomeKeyPair, SomeKeyPairRef},
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::{StaticPinnedFuture, pin_static_future};
use ulid::Ulid;

pub struct MockKeyPairRepository {
    keypairs: Arc<DashMap<Ulid, SomeKeyPair>>,
}

pub struct MockKeyPairRepositoryWithTransaction {
    keypairs: Arc<DashMap<Ulid, SomeKeyPair>>,
}

impl MockKeyPairRepository {
    pub fn new() -> Self {
        MockKeyPairRepository {
            keypairs: Arc::new(DashMap::new()),
        }
    }
}

impl KeyPairRepository for MockKeyPairRepository {
    fn start_transaction(
        &self,
    ) -> StaticPinnedFuture<Box<dyn KeyPairRepositoryWithTransaction>, KeyPairRepositoryError> {
        let keypairs_clone = self.keypairs.clone();
        pin_static_future(async move {
            Ok(Box::new(MockKeyPairRepositoryWithTransaction {
                keypairs: keypairs_clone,
            }) as Box<dyn KeyPairRepositoryWithTransaction>)
        })
    }

    fn get_by_id(
        &self,
        id: &Identifier<Ulid, SomeKeyPair>,
    ) -> StaticPinnedFuture<Option<SomeKeyPair>, KeyPairRepositoryError> {
        todo!()
    }

    fn get_active(&self) -> StaticPinnedFuture<Option<KeyPair<Active>>, KeyPairRepositoryError> {
        todo!()
    }

    fn save(&self, keypair: SomeKeyPairRef) -> StaticPinnedFuture<(), KeyPairRepositoryError> {
        todo!()
    }
}

impl KeyPairRepositoryWithTransaction for MockKeyPairRepositoryWithTransaction {
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), KeyPairRepositoryError> {
        todo!()
    }

    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), KeyPairRepositoryError> {
        todo!()
    }

    fn get_by_id(
        self: Box<Self>,
        id: &Identifier<Ulid, SomeKeyPair>,
    ) -> StaticPinnedFuture<
        (
            Box<dyn KeyPairRepositoryWithTransaction>,
            Option<SomeKeyPair>,
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
        keypair: SomeKeyPairRef,
    ) -> StaticPinnedFuture<(Box<dyn KeyPairRepositoryWithTransaction>, ()), KeyPairRepositoryError>
    {
        todo!()
    }
}
