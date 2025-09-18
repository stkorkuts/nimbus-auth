use mockall::mock;

mock! {
    pub TestKeyPairRepository {}

    impl nimbus_auth_application::services::keypair_repository::KeyPairRepository for TestKeyPairRepository {
        fn start_transaction(
            &self,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<Box<dyn nimbus_auth_application::services::keypair_repository::KeyPairRepositoryWithTransaction> ,nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError> ;
        fn get_by_id(
            &self,
            id: &nimbus_auth_domain::value_objects::identifier::Identifier<ulid::Ulid,nimbus_auth_domain::entities::keypair::KeyPair<nimbus_auth_domain::entities::keypair::Uninitialized>  > ,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<Option<nimbus_auth_domain::entities::keypair::InitializedKeyPair> ,nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError> ;
        fn get_active(&self) -> nimbus_auth_shared::futures::StaticPinnedFuture<Option<nimbus_auth_domain::entities::keypair::KeyPair<nimbus_auth_domain::entities::keypair::Active>  > ,nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError> ;
        fn save<'a>(&'a self, keypair: nimbus_auth_domain::entities::keypair::InitializedKeyPairRef<'a>) -> nimbus_auth_shared::futures::StaticPinnedFuture<(),nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError> ;
    }
}

mock! {
    pub TestKeyPairRepositoryWithTransaction {}

    impl nimbus_auth_application::services::keypair_repository::KeyPairRepositoryWithTransaction for TestKeyPairRepositoryWithTransaction {
        fn commit(self: Box<Self>) -> nimbus_auth_shared::futures::StaticPinnedFuture<(),nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError> ;
        fn rollback(self: Box<Self>) -> nimbus_auth_shared::futures::StaticPinnedFuture<(),nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError> ;
        fn get_by_id(
            self: Box<Self>,
            id: &nimbus_auth_domain::value_objects::identifier::Identifier<ulid::Ulid,nimbus_auth_domain::entities::keypair::KeyPair<nimbus_auth_domain::entities::keypair::Uninitialized> > ,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<(Box<dyn nimbus_auth_application::services::keypair_repository::KeyPairRepositoryWithTransaction> ,Option<nimbus_auth_domain::entities::keypair::InitializedKeyPair> ,),nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError, > ;
        fn get_active(
            self: Box<Self>,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<(Box<dyn nimbus_auth_application::services::keypair_repository::KeyPairRepositoryWithTransaction> ,Option<nimbus_auth_domain::entities::keypair::KeyPair<nimbus_auth_domain::entities::keypair::Active>  > ,),nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError, > ;
        fn save<'a>(
            self: Box<Self>,
            keypair: nimbus_auth_domain::entities::keypair::InitializedKeyPairRef<'a>,
        ) -> nimbus_auth_shared::futures::StaticPinnedFuture<(Box<dyn nimbus_auth_application::services::keypair_repository::KeyPairRepositoryWithTransaction> ,()),nimbus_auth_application::services::keypair_repository::errors::KeyPairRepositoryError> ;
    }
}
