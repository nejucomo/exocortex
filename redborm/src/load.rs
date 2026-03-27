use redb::{ReadTransaction, Value};

use crate::{OrmResult, RowValue};

/// A type which can be loaded (transitively) from a <u>k</u>ey <u>o</u>r <u>v</u>alue.
pub trait Load: Sized {
    /// The <u>K</u>ey <u>O</u>r <u>V</u>alue to load from
    type KOV: Value;

    /// Load a [Self] given `txn` and an `intermediate` to load from
    fn load_from(txn: &ReadTransaction, kov: <Self::KOV as Value>::SelfType<'_>)
    -> OrmResult<Self>;
}

impl<B> Load for B
where
    B: RowValue<Key: Clone>,
{
    type KOV = B::Key;

    fn load_from(txn: &ReadTransaction, key: B::Key) -> OrmResult<Self> {
        use crate::ext::ReadableTableExt as _;

        let tab = txn.open_table(Self::table_definition())?;
        tab.get_row(key)
    }
}
