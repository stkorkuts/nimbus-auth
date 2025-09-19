use std::sync::Arc;

use dashmap::DashMap;
use nimbus_auth_application::services::user_repository::{
    UserRepository, UserRepositoryWithTransaction, errors::UserRepositoryError,
};
use nimbus_auth_domain::{
    entities::{
        Entity,
        session::{Active, Session},
        user::{User, value_objects::name::UserName},
    },
    value_objects::identifier::{Identifier, IdentifierOfType},
};
use nimbus_auth_shared::futures::{StaticPinnedFuture, pin_static_future};
use ulid::Ulid;

pub struct MockUserRepository {
    users: Arc<DashMap<Identifier<Ulid, User>, User>>,
}

pub struct MockUserRepositoryWithTransaction {
    users: Arc<DashMap<Identifier<Ulid, User>, User>>,
}

impl MockUserRepository {
    pub fn new(users: Option<Vec<User>>) -> Self {
        let users = Arc::new(
            users
                .unwrap_or_default()
                .into_iter()
                .map(|user| (user.id().clone(), user))
                .collect(),
        );
        MockUserRepository { users }
    }
}

impl UserRepository for MockUserRepository {
    fn start_transaction(
        &self,
    ) -> StaticPinnedFuture<Box<dyn UserRepositoryWithTransaction>, UserRepositoryError> {
        let users_clone = self.users.clone();
        pin_static_future(async move {
            Ok(
                Box::new(MockUserRepositoryWithTransaction { users: users_clone })
                    as Box<dyn UserRepositoryWithTransaction>,
            )
        })
    }

    fn get_by_id(
        &self,
        id: Identifier<Ulid, User>,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
    }

    fn get_by_name(
        &self,
        user_name: &UserName,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
    }

    fn get_by_session(
        &self,
        session: &Session<Active>,
    ) -> StaticPinnedFuture<Option<User>, UserRepositoryError> {
        todo!()
    }

    fn save(&self, user: &User) -> StaticPinnedFuture<(), UserRepositoryError> {
        todo!()
    }
}

impl UserRepositoryWithTransaction for MockUserRepositoryWithTransaction {
    fn commit(self: Box<Self>) -> StaticPinnedFuture<(), UserRepositoryError> {
        todo!()
    }

    fn rollback(self: Box<Self>) -> StaticPinnedFuture<(), UserRepositoryError> {
        todo!()
    }

    fn get_by_id(
        self: Box<Self>,
        id: Identifier<Ulid, User>,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    > {
        todo!()
    }

    fn get_by_name(
        self: Box<Self>,
        user_name: &UserName,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    > {
        todo!()
    }

    fn get_by_session(
        self: Box<Self>,
        session: &Session<Active>,
    ) -> StaticPinnedFuture<
        (Box<dyn UserRepositoryWithTransaction>, Option<User>),
        UserRepositoryError,
    > {
        todo!()
    }

    fn save(
        self: Box<Self>,
        user: &User,
    ) -> StaticPinnedFuture<(Box<dyn UserRepositoryWithTransaction>, ()), UserRepositoryError> {
        todo!()
    }
}
