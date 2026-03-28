use redb::WriteTransaction;

use crate::{OrmResult, OwnedValue, RowValue};

/// Any value which can be (transitively) stored via a [WriteTransaction]
pub trait Store: Sized {
    /// The <u>K</u>ey <u>O</u>r <u>V</u>alue to load from
    type KOV: OwnedValue;

    /// Store this type into the db via the given `txn`
    fn store_into(self, txn: &WriteTransaction) -> OrmResult<Self::KOV>;
}

impl<B> Store for B
where
    B: RowValue<Key: From<u64> + Clone>,
{
    type KOV = B::Key;

    fn store_into(self, txn: &WriteTransaction) -> OrmResult<Self::KOV> {
        use crate::ext::TableExt as _;

        let mut tab = txn.open_table(Self::table_definition())?;
        tab.append_row(self)
    }
}
