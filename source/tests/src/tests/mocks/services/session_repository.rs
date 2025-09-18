use mockall::mock;

mock! {
    pub TestSessionRepository {}

    impl nimbus_auth_application::services::session_repository::SessionRepository for TestSessionRepository {
        fn start_transaction(
            &self,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<Box<dyn nimbus_auth_application::services::session_repository::SessionRepositoryWithTransaction> ,nimbus_auth_application::services::session_repository::errors::SessionRepositoryError> ;
        fn get_by_id(
            &self,
            id: nimbus_auth_domain::value_objects::identifier::Identifier<ulid::Ulid,nimbus_auth_domain::entities::session::Session<nimbus_auth_domain::entities::session::Uninitialized>  > ,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<Option<nimbus_auth_domain::entities::session::InitializedSession> ,nimbus_auth_application::services::session_repository::errors::SessionRepositoryError> ;
        fn save<'a>(
            &'a self,
            session: nimbus_auth_domain::entities::session::InitializedSessionRef<'a>,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<(),nimbus_auth_application::services::session_repository::errors::SessionRepositoryError> ;
    }
}

mock! {
    pub TestSessionRepositoryWithTransaction {}

    impl nimbus_auth_application::services::session_repository::SessionRepositoryWithTransaction for TestSessionRepositoryWithTransaction {
        fn commit(self: Box<Self>) -> nimbus_auth_shared::futures::StaticPinnedFuture<(),nimbus_auth_application::services::session_repository::errors::SessionRepositoryError> ;
        fn rollback(self: Box<Self>) -> nimbus_auth_shared::futures::StaticPinnedFuture<(), nimbus_auth_application::services::session_repository::errors::SessionRepositoryError>;
        fn get_by_id(
            self: Box<Self>,
            id: nimbus_auth_domain::value_objects::identifier::Identifier<ulid::Ulid,nimbus_auth_domain::entities::session::Session<nimbus_auth_domain::entities::session::Uninitialized>  > ,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
            (
                Box<dyn nimbus_auth_application::services::session_repository::SessionRepositoryWithTransaction>,
                Option<nimbus_auth_domain::entities::session::InitializedSession>,
            ),
            nimbus_auth_application::services::session_repository::errors::SessionRepositoryError,
        >;
        fn save<'a>(
            self: Box<Self>,
            session: nimbus_auth_domain::entities::session::InitializedSessionRef<'a>,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<(Box<dyn nimbus_auth_application::services::session_repository::SessionRepositoryWithTransaction>, ()), nimbus_auth_application::services::session_repository::errors::SessionRepositoryError>;
    }
}
