use std::ops::RangeBounds;

use redb::ReadableTable;

use crate::{OrmError, OrmResult, RowValue};

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

    /// Iterate over a range of [Row]s
    fn iter_row_range(
        &self,
        range: impl RangeBounds<Row::Key>,
    ) -> OrmResult<impl DoubleEndedIterator<Item = OrmResult<(Row::Key, Row)>>> {
        let range = self.range(range)?;
        Ok(range.map(|kvgres| {
            kvgres
                .map(|(kg, vg)| (kg.value(), vg.value()))
                .map_err(OrmError::from)
        }))
    }

    /// Iterate over all [Row]s
    fn iter_rows(&self) -> OrmResult<impl DoubleEndedIterator<Item = OrmResult<(Row::Key, Row)>>> {
        self.iter_row_range(..)
    }
}

impl<B, Row> ReadableTableExt<Row> for B
where
    B: ReadableTable<Row::Key, Row>,
    Row: RowValue,
{
}
