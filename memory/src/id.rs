use std::cmp::Ordering;
use std::marker::PhantomData;

use derive_new::new;

/// A locally unique identifier for a `T` value with [u64] representation
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
    pub fn transmute<U: ?Sized + 'static>(self) -> Id<U> {
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
        std::fmt::Display::fmt(&self.n, f)
    }
}

impl<T: ?Sized + 'static> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.n;
        let tname = std::any::type_name::<T>();
        let tnick = tname.rsplit_once("::").map(|(_, s)| s).unwrap_or(tname);
        write!(f, "Id:{tnick}({id})")
    }
}

impl<T: ?Sized + 'static> From<u64> for Id<T> {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}
