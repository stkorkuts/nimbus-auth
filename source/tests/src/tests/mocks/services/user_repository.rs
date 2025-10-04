use std::sync::Arc;

use nimbus_auth_application::services::user_repository::{
    UserRepository, UserRepositoryWithTransaction, errors::UserRepositoryError,
};
use nimbus_auth_domain::{
    entities::{
        Entity,
        session::{Active, Session},
        user::{User, value_objects::user_name::UserName},
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::{StaticPinnedFuture, pin_static_future};
use tokio::sync::Mutex;
use ulid::Ulid;

use crate::tests::mocks::datastore::MockDatastore;

pub struct MockUserRepository {
    datastore: Arc<MockDatastore>,
}

struct UserSave {
    old: Option<User>,
    new: User,
}

/// Represents mock user repository with active transaction
///
/// Transaction implemented with `ReadUncomitted` isolation level which is sufficient for tests for now
pub struct MockUserRepositoryWithTransaction {
    datastore: Arc<MockDatastore>,
    user_saves: Arc<Mutex<Vec<UserSave>>>,
}

impl MockUserRepository {
    pub fn new(datastore: Arc<MockDatastore>) -> Self {
        MockUserRepository { datastore }
    }
}

impl UserRepository for MockUserRepository {
    fn start_transaction(
        &self,
    ) -> StaticPinnedFuture<Box<dyn UserRepositoryWithTransaction>, UserRepositoryError> {
        let datastore_clone: Arc<MockDatastore> = self.datastore.clone();
        pin_static_future(async move {
            Ok(Box::new(MockUserRepositoryWithTransaction {
                datastore: datastore_clone,
                user_saves: Arc::new(Mutex::new(Vec::new())),
            }) as Box<dyn UserRepositoryWithTransaction>)
        })
    }

    fn get_by_id(
        &self,
        id: &Identifier<Ulid, User>,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError> {
        let datastore_clone: Arc<MockDatastore> = self.datastore.clone();
        let id_clone = id.clone();
        pin_static_future(async move {
            Ok(datastore_clone
                .users()
                .get(&id_clone)
                .map(|user_ref| user_ref.value().clone()))
        })
    }

    fn get_by_name(
        &self,
        user_name: &UserName,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError> {
        let datastore_clone: Arc<MockDatastore> = self.datastore.clone();
        let user_name_value = user_name.value().to_string();
        pin_static_future(async move {
            Ok(datastore_clone
                .users()
                .iter()
                .find(|entry| entry.name().value() == user_name_value)
                .map(|user_ref| user_ref.value().clone()))
        })
    }

    fn get_by_session(
        &self,
        session: &Session<Active>,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError> {
        let datastore_clone: Arc<MockDatastore> = self.datastore.clone();
        let user_id = session.user_id().clone();
        pin_static_future(async move {
            Ok(datastore_clone
                .users()
                .get(&user_id)
                .map(|user_ref| user_ref.value().clone()))
        })
    }

    fn save(&self, user: &User) -> StaticPinnedFuture<(), UserRepositoryError> {
        let datastore_clone: Arc<MockDatastore> = self.datastore.clone();
        let user_clone = user.clone();
        pin_static_future(async move {
            datastore_clone
                .users()
                .insert(user_clone.id().clone(), user_clone);
            Ok(())
        })
    }
}

impl UserRepositoryWithTransaction for MockUserRepositoryWithTransaction {
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), UserRepositoryError> {
        pin_static_future(async { Ok(()) })
    }

    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), UserRepositoryError> {
        pin_static_future(async move {
            let mut saves = self.user_saves.lock().await;
            let users = self.datastore.users();
            while let Some(save) = saves.pop() {
                match save.old {
                    Some(old) => {
                        users.insert(old.id().clone(), old.clone());
                    }
                    None => {
                        users.remove(save.new.id());
                    }
                }
            }
            Ok(())
        })
    }

    fn get_by_id(
        self: Box<Self>,
        id: &Identifier<Ulid, User>,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    > {
        let id_clone = id.clone();
        pin_static_future(async move {
            let user = self
                .datastore
                .users()
                .get(&id_clone)
                .map(|user_ref| user_ref.value().clone());
            Ok((self as Box<dyn UserRepositoryWithTransaction>, user))
        })
    }

    fn get_by_name(
        self: Box<Self>,
        user_name: &UserName,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    > {
        let user_name_value = user_name.value().to_string();
        pin_static_future(async move {
            let user = self
                .datastore
                .users()
                .iter()
                .find(|entry| entry.name().value() == user_name_value)
                .map(|user_ref| user_ref.value().clone());
            Ok((self as Box<dyn UserRepositoryWithTransaction>, user))
        })
    }

    fn get_by_session(
        self: Box<Self>,
        session: &Session<Active>,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    > {
        let user_id = session.user_id().clone();
        pin_static_future(async move {
            let user = self
                .datastore
                .users()
                .get(&user_id)
                .map(|user_ref| user_ref.value().clone());
            Ok((self as Box<dyn UserRepositoryWithTransaction>, user))
        })
    }

    fn save(
        self: Box<Self>,
        user: &User,
    ) -> StaticPinnedFuture<(Box<dyn UserRepositoryWithTransaction>, ()), UserRepositoryError> {
        let user_clone = user.clone();
        pin_static_future(async move {
            let old = self
                .datastore
                .users()
                .insert(user_clone.id().clone(), user_clone.clone());

            let save_record = UserSave {
                old,
                new: user_clone,
            };

            {
                let mut saves = self.user_saves.lock().await;
                saves.push(save_record);
            }

            Ok((self as Box<dyn UserRepositoryWithTransaction>, ()))
        })
    }
}
