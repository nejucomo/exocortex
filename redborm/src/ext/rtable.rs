use redb::ReadableTable;

use crate::{OrmResult, RowValue};

/// A convenience extension of [ReadableTable] for [RowValue]s
pub trait ReadableTableExt<Row: RowValue>: ReadableTable<Row::Key, Row> {
    /// Get a row value, or [None] if not present
    fn get_row_opt(&self, key: Row::Key) -> OrmResult<Option<Row>> {
        let optg = self.get(key)?;
        Ok(optg.map(|g| g.value()))
    }

    /// Get a row value, or [Err] if not present
    fn get_row(&self, key: Row::Key) -> OrmResult<Row>
    where
        Row::Key: Clone + std::fmt::Debug,
    {
        use crate::OrmError::UnknownKey;

        let optrow = self.get_row_opt(key.clone())?;
        optrow.ok_or_else(|| UnknownKey(format!("{key:?}")))
    }

    /*
    fn get_row_range(&self, range: impl RangeBounds<Row::Key>) -> OrmResult<RowRange<Row>> {
        let range = self.range(range)?;
        Ok(RowRange::new(range))
    }
    */
}

impl<B, Row> ReadableTableExt<Row> for B
where
    B: ReadableTable<Row::Key, Row>,
    Row: RowValue,
{
}
