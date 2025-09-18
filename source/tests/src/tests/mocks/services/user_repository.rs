use mockall::mock;

mock! {
    pub TestUserRepository {}

    impl nimbus_auth_application::services::user_repository::UserRepository for TestUserRepository {
        fn start_transaction(
            &self,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<Box<dyn nimbus_auth_application::services::user_repository::UserRepositoryWithTransaction>,nimbus_auth_application::services::user_repository::errors::UserRepositoryError> ;
        fn get_by_id(
            &self,
            id: nimbus_auth_domain::value_objects::identifier::Identifier<ulid::Ulid,nimbus_auth_domain::entities::user::User> ,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<Option<nimbus_auth_domain::entities::user::User> ,nimbus_auth_application::services::user_repository::errors::UserRepositoryError> ;
        fn get_by_name(
            &self,
            user_name: &nimbus_auth_domain::entities::user::value_objects::name::UserName,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<Option<nimbus_auth_domain::entities::user::User> ,nimbus_auth_application::services::user_repository::errors::UserRepositoryError> ;
        fn get_by_session(
            &self,
            session: &nimbus_auth_domain::entities::session::Session<nimbus_auth_domain::entities::session::Active> ,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<Option<nimbus_auth_domain::entities::user::User> ,nimbus_auth_application::services::user_repository::errors::UserRepositoryError> ;
        fn save(&self, user: &nimbus_auth_domain::entities::user::User) -> nimbus_auth_shared::futures::StaticPinnedFuture<(),nimbus_auth_application::services::user_repository::errors::UserRepositoryError> ;
    }
}

mock! {
    pub TestUserRepositoryWithTransaction {}

    impl nimbus_auth_application::services::user_repository::UserRepositoryWithTransaction for TestUserRepositoryWithTransaction {
        fn commit(self: Box<Self>) -> nimbus_auth_shared::futures::StaticPinnedFuture<(),nimbus_auth_application::services::user_repository::errors::UserRepositoryError> ;
        fn rollback(self: Box<Self>) -> nimbus_auth_shared::futures::StaticPinnedFuture<(), nimbus_auth_application::services::user_repository::errors::UserRepositoryError>;
        fn get_by_id(
            self: Box<Self>,
            id: nimbus_auth_domain::value_objects::identifier::Identifier<ulid::Ulid,nimbus_auth_domain::entities::user::User> ,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
            (Box<dyn nimbus_auth_application::services::user_repository::UserRepositoryWithTransaction>, Option<nimbus_auth_domain::entities::user::User>),
            nimbus_auth_application::services::user_repository::errors::UserRepositoryError,
        >;
        fn get_by_name(
            self: Box<Self>,
            user_name: &nimbus_auth_domain::entities::user::value_objects::name::UserName,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
            (Box<dyn nimbus_auth_application::services::user_repository::UserRepositoryWithTransaction>, Option<nimbus_auth_domain::entities::user::User>),
            nimbus_auth_application::services::user_repository::errors::UserRepositoryError,
        >;
        fn get_by_session(
            self: Box<Self>,
            session: &nimbus_auth_domain::entities::session::Session<nimbus_auth_domain::entities::session::Active> ,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
            (Box<dyn nimbus_auth_application::services::user_repository::UserRepositoryWithTransaction>, Option<nimbus_auth_domain::entities::user::User>),
            nimbus_auth_application::services::user_repository::errors::UserRepositoryError,
        >;
        fn save(
            self: Box<Self>,
            user: &nimbus_auth_domain::entities::user::User,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<(Box<dyn nimbus_auth_application::services::user_repository::UserRepositoryWithTransaction>, ()), nimbus_auth_application::services::user_repository::errors::UserRepositoryError>;
    }
}
