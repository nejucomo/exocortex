use std::path::Path;

use redb::Database;

use crate::{CardView, DamoResult, Id, Provider};

#[derive(Debug)]
pub struct RedProvider {
    #[allow(dead_code)]
    db: Database,
}

impl RedProvider {
    pub fn open_or_create<P>(dbpath: P) -> DamoResult<Self>
    where
        P: AsRef<Path>,
    {
        let db = Database::create(dbpath)?;
        Ok(RedProvider { db })
    }
}

impl Provider for RedProvider {
    fn is_empty(&self) -> DamoResult<bool> {
        todo!()
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
