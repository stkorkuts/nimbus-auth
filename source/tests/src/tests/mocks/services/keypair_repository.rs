use nimbus_auth_application::services::keypair_repository::KeyPairRepository;

pub struct MockKeyPairRepository {}

impl KeyPairRepository for MockKeyPairRepository {
    fn start_transaction(
        &self,
    ) -> nimbus_auth_shared::futures::StaticPinnedFuture<Box<dyn nimbus_auth_application::services::keypair_repository::KeyPairRepositoryWithTransaction>, nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError>{
        todo!()
    }

    fn get_by_id(
        &self,
        id: &nimbus_auth_domain::value_objects::identifier::Identifier<
            ulid::Ulid,
            nimbus_auth_domain::entities::keypair::KeyPair<
                nimbus_auth_domain::entities::keypair::Uninitialized,
            >,
        >,
    ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
        Option<nimbus_auth_domain::entities::keypair::InitializedKeyPair>,
        nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError,
    > {
        todo!()
    }

    fn get_active(
        &self,
    ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
        Option<
            nimbus_auth_domain::entities::keypair::KeyPair<
                nimbus_auth_domain::entities::keypair::Active,
            >,
        >,
        nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError,
    > {
        todo!()
    }

    fn save(
        &self,
        keypair: nimbus_auth_domain::entities::keypair::InitializedKeyPairRef,
    ) -> nimbus_auth_shared::futures::StaticPinnedFuture<
        (),
        nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError,
    > {
        todo!()
    }
}
