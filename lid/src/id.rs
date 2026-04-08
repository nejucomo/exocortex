use std::cmp::Ordering;
use std::marker::PhantomData;

use disqualified::ShortName;

#[doc = include_str!("id-synopsis.md")]
pub struct Id<T: ?Sized> {
    pub(crate) n: u64,
    ph: PhantomData<T>,
}

impl<T: ?Sized> Id<T> {
    /// Unwrap the bare [u64]
    pub fn unwrap(self) -> u64 {
        self.n
    }

    /// Produce a new [Id] by treating `self` as a "next unallocated [Id]" tracker
    pub fn inc_alloc(&mut self) -> Self {
        let id = *self;
        self.n += 1;
        id
    }

    /// Convert this `Id` to another type with identical value representation
    pub fn transmute<U>(self) -> Id<U> {
        Id::<U>::from(self.unwrap())
    }
}

impl<T: ?Sized> From<u64> for Id<T> {
    fn from(n: u64) -> Self {
        Id { n, ph: PhantomData }
    }
}

impl<T: ?Sized> Copy for Id<T> {}

impl<T: ?Sized> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Eq for Id<T> {}

impl<T: ?Sized> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl<T: ?Sized> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.n.cmp(&other.n)
    }
}

impl<T: ?Sized> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: ?Sized> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl<T: ?Sized> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.n;
        let nick = ShortName::of::<T>();
        write!(f, "Id({id} <{nick}>)")
    }
}
