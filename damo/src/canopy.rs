use std::path::Path;

use canopydb::{Database, Environment, Error};
use time::OffsetDateTime;

use crate::errors::UnknownId;
use crate::{Id, Provider};

#[derive(Debug)]
pub struct CanopyProvider {
    #[allow(dead_code)]
    db: Database,
}

impl CanopyProvider {
    pub fn open_or_create<P>(dbpath: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let env = Environment::new(dbpath)?;
        let db = env.get_or_create_database(&format!("{}_v0", env!("CARGO_PKG_NAME")))?;
        Ok(CanopyProvider { db })
    }
}

#[allow(unused_variables)]
impl Provider for CanopyProvider {
    fn is_empty(&self) -> bool {
        todo!()
    }

    fn card_new(&mut self) -> Result<Id, UnknownId> {
        todo!()
    }

    fn card_prev(&self, optfrom: Option<Id>) -> Result<Option<Id>, UnknownId> {
        todo!()
    }

    fn card_get_time_of_creation(&self, card: Id) -> Result<OffsetDateTime, UnknownId> {
        todo!()
    }

    fn card_get_synopsis(&self, card: Id) -> Result<&str, UnknownId> {
        todo!()
    }

    fn card_set_synopsis(&mut self, card: Id, synopsis: &str) -> Result<(), UnknownId> {
        todo!()
    }
}
