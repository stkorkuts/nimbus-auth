use std::marker::PhantomData;

use ulid::Ulid;

use crate::entities::Entity;

pub trait IdentifierOfType<TId> {
    fn new() -> Self;
    fn value(&self) -> &TId;
}

pub struct Identifier<TValue, TEntity: Entity<TValue>> {
    _marker: PhantomData<TEntity>,
    value: TValue,
}

impl<TValue: Clone, TEntity: Entity<TValue>> Clone for Identifier<TValue, TEntity> {
    fn clone(&self) -> Self {
        Self {
            _marker: self._marker,
            value: self.value.clone(),
        }
    }
}

impl<TEntity: Entity<Ulid>> IdentifierOfType<Ulid> for Identifier<Ulid, TEntity> {
    fn new() -> Self {
        Self {
            _marker: PhantomData,
            value: Ulid::new(),
        }
    }

    fn value(&self) -> &Ulid {
        &self.value
    }
}

impl<TEntity: Entity<Ulid>> Identifier<Ulid, TEntity> {
    pub fn from(value: Ulid) -> Self {
        Self {
            _marker: PhantomData,
            value,
        }
    }
}

impl<TEntity: Entity<Ulid>> ToString for Identifier<Ulid, TEntity> {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
