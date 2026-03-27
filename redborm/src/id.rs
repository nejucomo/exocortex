use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;

use derive_new::new;
use redb::{Key, TypeName, Value};

/// A locally unique identifier for a `T` value
#[derive(new)]
pub struct Id<T: ?Sized + 'static> {
    n: u64,
    #[new(default)]
    ph: PhantomData<T>,
}

impl<T: ?Sized + 'static> Id<T> {
    /// Produce a new [Id] by treating `self` as a "next unallocated [Id]" tracker
    pub fn alloc(&mut self) -> Self {
        let id = *self;
        self.n += 1;
        id
    }

    /// The next id, one larger
    pub fn inc(self) -> Self {
        Id::new(self.n + 1)
    }

    /// Convert to a different [Id] type
    #[allow(dead_code)]
    pub(crate) fn transmute<U: ?Sized + 'static>(self) -> Id<U> {
        Id::new(self.n)
    }
}

impl<T: ?Sized + 'static> Copy for Id<T> {}

impl<T: ?Sized + 'static> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized + 'static> Default for Id<T> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<T: ?Sized + 'static> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl<T: ?Sized + 'static> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.n;
        let tname = std::any::type_name::<T>();
        let tnick = tname.rsplit_once("::").map(|(_, s)| s).unwrap_or(tname);
        write!(f, "Id:{tnick}({id})")
    }
}

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
        let bytes: [u8; 8] = data.try_into().expect("Id<T> must be 8 bytes");
        Id::new(u64::from_be_bytes(bytes))
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        value.n.to_be_bytes()
    }

    fn type_name() -> TypeName {
        // Important trade-off:
        // - same name for all Id<T> => physically same schema across T
        // - distinct names per T => safer against mixing tables accidentally
        TypeName::new(std::any::type_name::<Id<T>>())
    }
}

impl<T: ?Sized + 'static> Key for Id<T> {
    fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
        let a = u64::from_be_bytes(data1.try_into().expect("Id<T> key must be 8 bytes"));
        let b = u64::from_be_bytes(data2.try_into().expect("Id<T> key must be 8 bytes"));
        a.cmp(&b)
    }
}
