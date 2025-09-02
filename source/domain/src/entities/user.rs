use ulid::Ulid;

use crate::{entities::Entity, value_objects::identifier::Identifier};

pub struct User {
    id: Identifier<Ulid, User>,
}

impl Entity<Ulid> for User {
    type Id = Identifier<Ulid, User>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
