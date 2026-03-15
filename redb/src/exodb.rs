use std::path::Path;

use redb::Database;

use crate::channel::{FromDb, ToDb, channel_pair};
use crate::messages::{RepSpec, Reply, ReqSpec, Request};
use crate::thread::run_db_thread;
use crate::{Id, IdTagged};

/// The `exocortex` database
#[derive(Debug)]
pub struct ExoDb {
    to_db: ToDb,
    from_db: FromDb,
}

impl ExoDb {
    /// Open or create a new database at the given path
    pub fn init<P>(dbpath: P) -> Result<Self, redb::DatabaseError>
    where
        P: AsRef<Path>,
    {
        let dbpath = dbpath.as_ref();
        std::fs::create_dir_all(dbpath.parent().unwrap())?;
        let redb = Database::create(dbpath)?;

        let (to_from_db, to_from_app) = channel_pair();
        std::thread::spawn(|| run_db_thread(redb, to_from_app));

        Ok(ExoDb::from(to_from_db))
    }

    /// Post a request and block on the reply
    pub fn request(&self, req: impl Into<ReqSpec>) -> RepSpec {
        // FIXME: This seems goofy; so maybe the "request id API" design needs change
        let reqid_in = Id::<ReqSpec>::new(0);
        self.post_request(IdTagged::new(Id::new(0), req.into()));
        let Reply { reqid, repspec } = self.from_db.recv().unwrap();
        assert_eq!(reqid.id, reqid_in.id);
        repspec
    }

    /// Post a request to the database
    pub fn post_request(&self, req: Request) {
        self.to_db.send(req).unwrap();
    }

    /// Poll for a reply from the database without blocking
    pub fn poll_reply(&self) -> Option<Reply> {
        use std::sync::mpsc::TryRecvError::*;

        self.from_db
            .try_recv()
            .map(Some)
            .unwrap_or_else(|e| match e {
                Empty => None,
                Disconnected => panic!("db disconnected"),
            })
    }
}

impl From<(ToDb, FromDb)> for ExoDb {
    fn from((to_db, from_db): (ToDb, FromDb)) -> Self {
        ExoDb { to_db, from_db }
    }
}
