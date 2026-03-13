mod tabdefs;

use std::path::Path;

use redb::{Database, ReadableDatabase as _, ReadableTableMetadata as _};

use crate::{CardView, DamoResult, Id, Provider};

use self::tabdefs::TableDefinitions;

#[derive(Debug)]
pub struct RedProvider {
    db: Database,
    tables: TableDefinitions,
}

impl RedProvider {
    pub fn open_or_create<P>(dbpath: P) -> DamoResult<Self>
    where
        P: AsRef<Path>,
    {
        let db = Database::create(dbpath)?;
        let tables = TableDefinitions::default();
        Ok(RedProvider { db, tables })
    }
}

impl Provider for RedProvider {
    fn is_empty(&self) -> DamoResult<bool> {
        let txn = self.db.begin_read()?;
        let tab = txn.open_table(self.tables.synopsis)?;
        let len = tab.len()?;
        Ok(len == 0)
    }

    fn card_new(&mut self) -> DamoResult<Id> {
        todo!()
    }

    fn card_set_synopsis(&mut self, card: Id, synopsis: &str) -> DamoResult<()> {
        let _ = (card, synopsis);
        todo!()
    }

    // Temporary stub:
    type CardScan<'a>
        = std::option::IntoIter<DamoResult<CardView<'a>>>
    where
        Self: 'a;

    fn card_scan(&self) -> DamoResult<Self::CardScan<'_>> {
        Ok(None.into_iter())
    }
}
