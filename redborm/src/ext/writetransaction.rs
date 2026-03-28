use extension_traits::extension;
use redb::WriteTransaction;

use crate::{OrmResult, Store};

/// Streamline [Store] ergonomics
#[extension(pub trait WriteTransactionExt)]
impl WriteTransaction {
    /// Store an `L`
    fn store<S: Store>(&self, val: S) -> OrmResult<S::KOV> {
        val.store_into(self)
    }
}
