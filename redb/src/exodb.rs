use std::path::Path;

use derive_new::new;
use redb::Database;

use crate::Id;
use crate::channel::{FromDb, ToDb, channel_pair};
use crate::messages::{RepSpec, Reply, ReqSpec, Request};
use crate::thread::run_db_thread;

/// The `exocortex` database
#[derive(Debug, new)]
pub struct ExoDb {
    to_db: ToDb,
    from_db: FromDb,
    #[new(default)]
    nextid: Id<Request>,
    #[new(default)]
    recvid: Option<Id<Request>>,
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

        let (to_db, from_db) = to_from_db;
        Ok(ExoDb::new(to_db, from_db))
    }

    /// Block on getting a reply from a given request
    ///
    /// # Panic
    ///
    /// This will panic if there are outstanding requests which have not been replied to yet.
    pub fn request(&mut self, req: impl Into<ReqSpec>) -> RepSpec {
        assert_eq!(self.nextid, self.recvid.unwrap_or_default());
        let reqid = self.post_request(req);
        let reply = self.wait_reply();
        // This is implied by the earlier `assert_eq`:
        assert_eq!(reqid, reply.reqid);
        reply.repspec
    }

    /// Post a request to the database
    pub fn post_request(&mut self, req: impl Into<ReqSpec>) -> Id<Request> {
        let id = self.nextid.alloc();
        self.to_db.send(Request::new(id, req.into())).unwrap();
        id
    }

    /// Poll for a reply from the database without blocking
    pub fn poll_reply(&mut self) -> Option<Reply> {
        use std::sync::mpsc::TryRecvError::*;

        self.from_db
            .try_recv()
            .map(|r| self.track_reply(r))
            .map(Some)
            .unwrap_or_else(|e| match e {
                Empty => None,
                Disconnected => panic!("db disconnected"),
            })
    }

    /// Block until a reply is received
    pub fn wait_reply(&mut self) -> Reply {
        self.track_reply(self.from_db.recv().unwrap())
    }

    fn track_reply(&mut self, reply: Reply) -> Reply {
        self.recvid = Some(reply.reqid);
        reply
    }
}
