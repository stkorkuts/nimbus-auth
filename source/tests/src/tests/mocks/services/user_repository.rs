use nimbus_auth_application::services::user_repository::UserRepository;

pub struct MockUserRepository {}

impl UserRepository for MockUserRepository {
    fn start_transaction(
        &self,
    ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
        Box<dyn nimbus_auth_application::services::user_repository::UserRepositoryWithTransaction>,
        nimbus_auth_application::services::user_repository::errors::UserRepositoryError,
    > {
        todo!()
    }

    fn get_by_id(
        &self,
        id: nimbus_auth_domain::value_objects::identifier::Identifier<
            ulid::Ulid,
            nimbus_auth_domain::entities::user::User,
        >,
    ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
        Option<nimbus_auth_domain::entities::user::User>,
        nimbus_auth_application::services::user_repository::errors::UserRepositoryError,
    > {
        todo!()
    }

    fn get_by_name(
        &self,
        user_name: &nimbus_auth_domain::entities::user::value_objects::name::UserName,
    ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
        Option<nimbus_auth_domain::entities::user::User>,
        nimbus_auth_application::services::user_repository::errors::UserRepositoryError,
    > {
        todo!()
    }

    fn get_by_session(
        &self,
        session: &nimbus_auth_domain::entities::session::Session<
            nimbus_auth_domain::entities::session::Active,
        >,
    ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
        Option<nimbus_auth_domain::entities::user::User>,
        nimbus_auth_application::services::user_repository::errors::UserRepositoryError,
    > {
        todo!()
    }

    fn save(
        &self,
        user: &nimbus_auth_domain::entities::user::User,
    ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
        (),
        nimbus_auth_application::services::user_repository::errors::UserRepositoryError,
    > {
        todo!()
    }
}
