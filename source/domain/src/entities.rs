use crate::value_objects::IdentifierOfType;

pub mod session;
pub mod user;

pub trait Entity<TId> {
    type Id: IdentifierOfType<TId>;
    fn id(&self) -> &Self::Id;
}
