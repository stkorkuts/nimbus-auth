use std::{fmt::Display, hash, marker::PhantomData};

use ulid::Ulid;

use crate::entities::Entity;

pub trait IdentifierOfType<TId> {
    fn new() -> Self;
    fn value(&self) -> &TId;
}

#[derive(Debug)]
pub struct Identifier<TValue, TEntity: Entity<TValue>> {
    _marker: PhantomData<TEntity>,
    value: TValue,
}

impl<TValue: PartialEq, TEntity: Entity<TValue>> PartialEq for Identifier<TValue, TEntity> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<TValue: Eq, TEntity: Entity<TValue>> Eq for Identifier<TValue, TEntity> {}

impl<TValue: hash::Hash, TEntity: Entity<TValue>> hash::Hash for Identifier<TValue, TEntity> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
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

impl<TEntity: Entity<Ulid>> Display for Identifier<Ulid, TEntity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
