#![doc = include_str!("id-synopsis.md")]
#![deny(unsafe_code, missing_docs)]

use std::cmp::Ordering;
use std::marker::PhantomData;

use disqualified::ShortName;

#[doc = include_str!("id-synopsis.md")]
pub struct Id<T: ?Sized + 'static> {
    n: u64,
    ph: PhantomData<T>,
}

impl<T: ?Sized + 'static> Id<T> {
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
}

impl<T: ?Sized + 'static> From<u64> for Id<T> {
    fn from(n: u64) -> Self {
        Id { n, ph: PhantomData }
    }
}

impl<T: ?Sized + 'static> Copy for Id<T> {}

impl<T: ?Sized + 'static> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized + 'static> Eq for Id<T> {}

impl<T: ?Sized + 'static> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl<T: ?Sized + 'static> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.n.cmp(&other.n)
    }
}

impl<T: ?Sized + 'static> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: ?Sized + 'static> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl<T: ?Sized + 'static> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.n;
        let nick = ShortName::of::<T>();
        write!(f, "Id({id} <{nick}>)")
    }
}

#[cfg(feature = "redb")]
mod redb_impls {
    use std::cmp::Ordering;

    use redb::{Key, TypeName, Value};

    use crate::Id;

    impl<T: ?Sized + 'static> Value for Id<T> {
        type SelfType<'a>
            = Id<T>
        where
            Self: 'a;
        type AsBytes<'a>
            = [u8; 8]
        where
            Self: 'a;

        fn fixed_width() -> Option<usize> {
            Some(8)
        }

        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
        where
            Self: 'a,
        {
            Id::from(u64_from_be_slice(data))
        }

        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
        where
            Self: 'b,
        {
            value.n.to_be_bytes()
        }

        fn type_name() -> TypeName {
            TypeName::new(std::any::type_name::<Self>())
        }
    }

    impl<T: ?Sized + 'static> Key for Id<T> {
        fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
            let a = u64_from_be_slice(data1);
            let b = u64_from_be_slice(data2);
            a.cmp(&b)
        }
    }

    fn u64_from_be_slice(data: &[u8]) -> u64 {
        u64::from_be_bytes(data.try_into().expect("Id<T> key must be 8 bytes"))
    }
}
