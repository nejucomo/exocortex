use std::path::Path;

use exocortex_memory::Provider;
use redb::Database;

use crate::{RedError, RedResult};

#[derive(Debug)]
pub struct RedMem {
    #[allow(dead_code)]
    redb: Database,
}

impl RedMem {
    /// Open or create a new database at the given path
    pub fn init<P>(dbpath: P) -> RedResult<Self>
    where
        P: AsRef<Path>,
    {
        let dbpath = dbpath.as_ref();
        log::info!("Opening database: {:?}", dbpath.display());
        std::fs::create_dir_all(dbpath.parent().unwrap())?;
        let redb = redb::Database::create(dbpath)?;
        Ok(Self { redb })
    }
}

impl Provider for RedMem {
    type Error = RedError;
}
