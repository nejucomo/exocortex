use redb::{ReadTransaction, Value};

use crate::ext::ReadableTableExt as _;
use crate::{OrmResult, RowValue};

/// A type which can be loaded (transitively) from a <u>k</u>ey <u>o</u>r <u>v</u>alue.
pub trait Load: Sized {
    /// The <u>K</u>ey <u>O</u>r <u>V</u>alue to load from
    type KOV: Value;

    /// Load a [Self] given `txn` and an `intermediate` to load from
    fn load_from(txn: &ReadTransaction, kov: <Self::KOV as Value>::SelfType<'_>)
    -> OrmResult<Self>;

    /// Scan [Self] items given a `txn`
    fn scan_from<F>(txn: &ReadTransaction, take_item: F) -> OrmResult<()>
    where
        F: FnMut(<Self::KOV as Value>::SelfType<'_>, Self) -> OrmResult<()>;
}

impl<B> Load for B
where
    B: RowValue<Key: Clone>,
{
    type KOV = B::Key;

    fn load_from(txn: &ReadTransaction, key: B::Key) -> OrmResult<Self> {
        let tab = txn.open_table(Self::table_definition())?;
        tab.get_row(key)
    }

    fn scan_from<F>(txn: &ReadTransaction, mut take_item: F) -> OrmResult<()>
    where
        F: FnMut(<Self::KOV as Value>::SelfType<'_>, Self) -> OrmResult<()>,
    {
        use redb::TableError::TableDoesNotExist;

        match txn.open_table(Self::table_definition()) {
            Ok(tab) => {
                for kvres in tab.iter_rows()? {
                    let (k, v) = kvres?;
                    take_item(k, v)?;
                }
                Ok(())
            }
            // Ok, just empty:
            Err(TableDoesNotExist(_)) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
