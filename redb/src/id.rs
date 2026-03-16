use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;

use derive_new::new;
use redb::{Key, TypeName, Value};

pub type IdNum = u64;

/// A locally unique identifier for a `T` value
#[derive(new)]
pub struct Id<T> {
    pub(crate) id: IdNum, // BUG: We need to kill id's for request identification
    #[new(default)]
    ph: PhantomData<T>,
}

impl<T> Id<T> {
    /// Produce a new [Id] by treating `self` as a "next unallocated [Id]" tracker
    pub fn alloc(&mut self) -> Self {
        let id = *self;
        self.id += 1;
        id
    }

    /// The next id, one larger
    pub fn inc(self) -> Self {
        Id::new(self.id + 1)
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.id;
        let tname = std::any::type_name::<T>();
        let tnick = tname.rsplit_once("::").map(|(_, s)| s).unwrap_or(tname);
        write!(f, "Id:{tnick}({id})")
    }
}

impl<T: 'static> Value for Id<T> {
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
        Id::new(u64::from_le_bytes(bytes))
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        value.id.to_le_bytes()
    }

    fn type_name() -> TypeName {
        // Important trade-off:
        // - same name for all Id<T> => physically same schema across T
        // - distinct names per T => safer against mixing tables accidentally
        TypeName::new(std::any::type_name::<Id<T>>())
    }
}

impl<T: 'static> Key for Id<T> {
    fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
        let a = u64::from_le_bytes(data1.try_into().expect("Id<T> key must be 8 bytes"));
        let b = u64::from_le_bytes(data2.try_into().expect("Id<T> key must be 8 bytes"));
        a.cmp(&b)
    }
}
