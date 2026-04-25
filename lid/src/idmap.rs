use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::collections::btree_map::{Entry, IntoIter};

use crate::{Id, ValueWithId as _, WithId};

/// Store `T` items indexed by their [Id]
#[derive(Debug, Default)]
pub struct IdMap<T>(BTreeMap<Id<T>, T>);

impl<T> IdMap<T> {
    /// Similer to [BTreeMap::entry]
    pub fn entry(&mut self, key: Id<T>) -> Entry<'_, Id<T>, T> {
        self.0.entry(key)
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
