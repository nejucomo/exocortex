use std::path::Path;

use exocortex_tserv::ThreadService;

use crate::handler::Handler as _;
use crate::messages::{DbReply, DbRequest};
use crate::{DbError, Result};

/// The `exocorted` database
#[derive(Debug)]
pub struct Database {
    redb: redb::Database,
}

/// The [ThreadService] type for the database
pub type DatabaseThreadService = ThreadService<DbRequest, DbReply, DbError>;

impl Database {
    /// Open or create a new database at the given path
    pub fn init<P>(dbpath: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let dbpath = dbpath.as_ref();
        std::fs::create_dir_all(dbpath.parent().unwrap())?;
        let redb = redb::Database::create(dbpath)?;
        Ok(Database { redb })
    }

    /// Handle a db Request
    pub fn handle(&mut self, request: DbRequest) -> Result<DbReply> {
        self.redb.handle(request)
    }

    /// Convert into a thread service
    pub fn launch_thread_service(mut self) -> DatabaseThreadService {
        ThreadService::launch(move |request: DbRequest| self.handle(request))
    }
}
