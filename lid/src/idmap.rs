use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::collections::btree_map::{Entry, IntoIter};

use thiserror::Error;

use crate::{Id, ValueWithId as _, WithId};

/// Store `T` items indexed by their [Id]
#[derive(Debug)]
pub struct IdMap<T>(BTreeMap<Id<T>, T>);

impl<T> Default for IdMap<T> {
    fn default() -> Self {
        Self(BTreeMap::default())
    }
}

/// Errors produced by [IdMap] usage
#[derive(Debug, Error)]
pub enum IdMapError<T> {
    /// Failed to insert an entry due to [Id] collision
    #[error("Id already present {id:?} with {prev:?} when inserting {new:?}")]
    DuplicateId {
        /// The conflicting [Id]
        id: Id<T>,
        /// The previous entry
        prev: T,
        /// The new entry
        new: T,
    },

    /// No entry for the given [Id]
    #[error("Unknown entry {0:?}")]
    UnknownEntry(Id<T>),
}

/// The results of [IdMap] operations [Box] the large error types
pub type IdMapResult<K, T> = Result<K, Box<IdMapError<T>>>;

impl<T> IdMap<T> {
    /// Insert a new entry
    pub fn insert_new(&mut self, id: Id<T>, value: T) -> IdMapResult<(), T> {
        match self.0.entry(id) {
            Entry::Vacant(ve) => {
                ve.insert(value);
                Ok(())
            }
            Entry::Occupied(oe) => Err(Box::new(IdMapError::DuplicateId {
                id,
                prev: oe.remove(),
                new: value,
            })),
        }
    }

    /// Get the entry with a given [Id]
    pub fn get_mut(&mut self, id: Id<T>) -> IdMapResult<&mut T, T> {
        self.0
            .get_mut(&id)
            .ok_or_else(|| Box::new(IdMapError::UnknownEntry(id)))
    }
}

impl<T> IntoIterator for IdMap<T> {
    type Item = WithId<T>;
    type IntoIter = WithIdIter<IntoIter<Id<T>, T>>;

    fn into_iter(self) -> Self::IntoIter {
        WithIdIter(self.0.into_iter())
    }
}

#[derive(Debug)]
pub struct WithIdIter<I>(I);

impl<I, D, T> Iterator for WithIdIter<I>
where
    I: Iterator<Item = (D, T)>,
    D: Borrow<Id<T>>,
{
    type Item = WithId<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(d, t)| t.with_id(*d.borrow()))
    }
}
