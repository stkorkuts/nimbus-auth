use nimbus_auth_domain::entities::keypair::InitializedKeyPair;
use nimbus_auth_shared::futures::PinnedFuture;
use ulid::Ulid;

pub trait KeyPairRepository: Send + Sync {
    fn get_by_id(&self, id: &Ulid) -> PinnedFuture<Option<InitializedKeyPair>>;
    fn save(&self, keypair: &InitializedKeyPair) -> PinnedFuture<()>;
}
