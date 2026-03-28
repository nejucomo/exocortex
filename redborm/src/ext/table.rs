use extension_traits::extension;
use redb::Table;

use crate::{OrmResult, RowValue};

#[extension(pub trait TableExt)]
impl<'txn, Row: RowValue> Table<'txn, Row::Key, Row> {
    /// Insert a [RowValue]
    fn insert_row(&mut self, key: Row::Key, row: Row) -> OrmResult<Option<Row>> {
        let optg = self.insert(key, row)?;
        Ok(optg.map(|g| g.value()))
    }

    /// Append a [RowValue]
    ///
    /// # Panics
    ///
    /// This assumes rows are never deleted. If they are deleted, keys will collide, causing a panic.
    fn append_row(&mut self, row: Row) -> OrmResult<Row::Key>
    where
        Row::Key: From<u64> + Clone,
    {
        use redb::ReadableTableMetadata as _;

        let keynum = self.len()?;
        let key = Row::Key::from(keynum);
        let optprev = self.insert_row(key.clone(), row)?;
        assert!(optprev.is_none(), "collision/re-use of key {keynum:?}");
        Ok(key)
    }
}
