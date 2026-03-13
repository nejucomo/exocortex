use std::path::Path;

use redb::Database;
use time::OffsetDateTime;

use crate::{DamoResult, Id, Provider};

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

#[allow(unused_variables)]
impl Provider for RedProvider {
    fn is_empty(&self) -> bool {
        todo!()
    }

    fn card_new(&mut self) -> DamoResult<Id> {
        todo!()
    }

    fn card_prev(&self, optfrom: Option<Id>) -> DamoResult<Option<Id>> {
        todo!()
    }

    fn card_get_time_of_creation(&self, card: Id) -> DamoResult<OffsetDateTime> {
        todo!()
    }

    fn card_get_synopsis(&self, card: Id) -> DamoResult<&str> {
        todo!()
    }

    fn card_set_synopsis(&mut self, card: Id, synopsis: &str) -> DamoResult<()> {
        todo!()
    }
}
