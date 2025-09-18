use nimbus_auth_application::services::session_repository::SessionRepository;

pub struct MockSessionRepository {}

impl SessionRepository for MockSessionRepository {
    fn start_transaction(
        &self,
    ) -> nimbus_auth_shared::futures::StaticPinnedFuture<Box<dyn nimbus_auth_application::services::session_repository::SessionRepositoryWithTransaction>, nimbus_auth_application::services::session_repository::errors::SessionRepositoryError>{
        todo!()
    }

    fn get_by_id(
        &self,
        id: nimbus_auth_domain::value_objects::identifier::Identifier<
            ulid::Ulid,
            nimbus_auth_domain::entities::session::Session<
                nimbus_auth_domain::entities::session::Uninitialized,
            >,
        >,
    ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
        Option<nimbus_auth_domain::entities::session::InitializedSession>,
        nimbus_auth_application::services::session_repository::errors::SessionRepositoryError,
    > {
        todo!()
    }

    fn save(
        &self,
        session: nimbus_auth_domain::entities::session::InitializedSessionRef,
    ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
        (),
        nimbus_auth_application::services::session_repository::errors::SessionRepositoryError,
    > {
        todo!()
    }
}
