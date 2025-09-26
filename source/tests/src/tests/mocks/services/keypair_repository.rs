use std::sync::Arc;

use dashmap::DashMap;
use nimbus_auth_application::services::keypair_repository::{
    KeyPairRepository, KeyPairRepositoryWithTransaction, errors::KeyPairRepositoryError,
};
use nimbus_auth_domain::{
    entities::{
        Entity,
        keypair::{Active, KeyPair, SomeKeyPair, SomeKeyPairRef},
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::{StaticPinnedFuture, pin_static_future};
use tokio::sync::Mutex;
use ulid::Ulid;

use crate::tests::mocks::datastore::MockDatastore;

pub struct MockKeyPairRepository {
    datastore: Arc<MockDatastore>,
}

struct KeyPairSave {
    old: Option<SomeKeyPair>,
    new: SomeKeyPair,
}

pub struct MockKeyPairRepositoryWithTransaction {
    datastore: Arc<MockDatastore>,
    keypair_saves: Arc<Mutex<Vec<KeyPairSave>>>,
}

impl MockKeyPairRepository {
    pub fn new(datastore: Arc<MockDatastore>) -> Self {
        MockKeyPairRepository { datastore }
    }
}

impl KeyPairRepository for MockKeyPairRepository {
    fn start_transaction(
        &self,
    ) -> StaticPinnedFuture<Box<dyn KeyPairRepositoryWithTransaction>, KeyPairRepositoryError> {
        let datastore_clone = self.datastore.clone();
        pin_static_future(async move {
            Ok(Box::new(MockKeyPairRepositoryWithTransaction {
                datastore: datastore_clone,
                keypair_saves: Arc::new(Mutex::new(Vec::new())),
            }) as Box<dyn KeyPairRepositoryWithTransaction>)
        })
    }

    fn get_by_id(
        &self,
        id: &Identifier<Ulid, SomeKeyPair>,
    ) -> StaticPinnedFuture<Option<SomeKeyPair>, KeyPairRepositoryError> {
        let datastore_clone: Arc<MockDatastore> = self.datastore.clone();
        let id_clone = id.clone();
        pin_static_future(async move {
            Ok(datastore_clone
                .keypairs()
                .get(&id_clone)
                .map(|keypair_ref| keypair_ref.value().clone()))
        })
    }

    fn get_active(&self) -> StaticPinnedFuture<Option<KeyPair<Active>>, KeyPairRepositoryError> {
        let datastore_clone: Arc<MockDatastore> = self.datastore.clone();
        pin_static_future(async move {
            Ok(datastore_clone
                .keypairs()
                .iter()
                .find_map(|entry| match entry.value() {
                    SomeKeyPair::Active { keypair, .. } => Some(keypair.clone()),
                    _ => None,
                }))
        })
    }

    fn save(&self, keypair: SomeKeyPairRef) -> StaticPinnedFuture<(), KeyPairRepositoryError> {
        let datastore_clone: Arc<MockDatastore> = self.datastore.clone();
        let keypair_clone = keypair.deref_clone();
        pin_static_future(async move {
            datastore_clone
                .keypairs()
                .insert(keypair_clone.id().clone(), keypair_clone);
            Ok(())
        })
    }
}

impl KeyPairRepositoryWithTransaction for MockKeyPairRepositoryWithTransaction {
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), KeyPairRepositoryError> {
        pin_static_future(async { Ok(()) })
    }

    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), KeyPairRepositoryError> {
        pin_static_future(async move {
            let mut saves = self.keypair_saves.lock().await;
            let keypairs = self.datastore.keypairs();
            while let Some(save) = saves.pop() {
                match save.old {
                    Some(old) => {
                        keypairs.insert(old.id().clone(), old.clone());
                    }
                    None => {
                        keypairs.remove(save.new.id());
                    }
                }
            }
            Ok(())
        })
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
        let id_clone = id.clone();
        pin_static_future(async move {
            let keypair = self
                .datastore
                .keypairs()
                .get(&id_clone)
                .map(|keypair_ref| keypair_ref.value().clone());
            Ok((self as Box<dyn KeyPairRepositoryWithTransaction>, keypair))
        })
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
        pin_static_future(async move {
            let keypair = self
                .datastore
                .keypairs()
                .iter()
                .find_map(|entry| match entry.value() {
                    SomeKeyPair::Active { keypair, .. } => Some(keypair.clone()),
                    _ => None,
                });
            Ok((self as Box<dyn KeyPairRepositoryWithTransaction>, keypair))
        })
    }

    fn save(
        self: Box<Self>,
        keypair: SomeKeyPairRef,
    ) -> StaticPinnedFuture<(Box<dyn KeyPairRepositoryWithTransaction>, ()), KeyPairRepositoryError>
    {
        let keypair_clone = keypair.deref_clone();
        pin_static_future(async move {
            let old = self
                .datastore
                .keypairs()
                .insert(keypair_clone.id().clone(), keypair_clone.clone());

            let save_record = KeyPairSave {
                old,
                new: keypair_clone,
            };

            {
                let mut saves = self.keypair_saves.lock().await;
                saves.push(save_record);
            }

            Ok((self as Box<dyn KeyPairRepositoryWithTransaction>, ()))
        })
    }
}
