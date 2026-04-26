use derive_more::{AsRef, Deref, DerefMut};

use crate::Id;

/// A value with its [Id]
#[derive(AsRef, Debug, Deref, DerefMut)]
pub struct WithId<T> {
    /// The [Id]
    pub id: Id<T>,
    /// The identified `value`
    #[as_ref]
    #[deref]
    #[deref_mut]
    pub value: T,
}

/// An extension of any [Sized] type to provide a cute `thing.with_id(id)` method
pub trait ValueWithId: Sized {
    /// Convert into the [WithId] for `self`, given `id`
    fn with_id(self, id: Id<Self>) -> WithId<Self> {
        WithId { id, value: self }
    }
}

impl<B> ValueWithId for B where B: Sized {}
