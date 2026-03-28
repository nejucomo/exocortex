use extension_traits::extension;
use redb::{ReadTransaction, Value};

use crate::{Load, OrmResult};

/// Streamline [Load] ergonomics
#[extension(pub trait ReadTransactionExt)]
impl ReadTransaction {
    /// Load an `L`
    fn load<L: Load>(&self, kov: <L::KOV as Value>::SelfType<'_>) -> OrmResult<L> {
        L::load_from(self, kov)
    }

    /// Scan all `L` values
    fn scan<L, F>(&self, take_item: F) -> OrmResult<()>
    where
        L: Load,
        F: FnMut(<L::KOV as Value>::SelfType<'_>, L) -> OrmResult<()>,
    {
        L::scan_from(self, take_item)
    }
}
