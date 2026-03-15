use std::path::Path;
use std::thread::JoinHandle;

use derive_new::new;
use redb::Database;

use crate::channel::{FromDb, ToDb, channel_pair};
use crate::messages::{RepSpec, Reply, ReqSpec, Request};
use crate::{DbError, DbResult, Id, dbthread};

/// The `exocortex` database
#[derive(Debug, new)]
#[new(visibility = "")]
pub struct ExoDb {
    #[new(into)]
    jh: Option<JoinHandle<DbResult<()>>>,
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
        let handle = dbthread::launch(redb, to_from_app);

        let (to_db, from_db) = to_from_db;
        Ok(ExoDb::new(handle, to_db, from_db))
    }

    /// Block on getting a reply from a given request
    ///
    /// # Panic
    ///
    /// This will panic if there are outstanding requests which have not been replied to yet.
    pub fn request(&mut self, req: impl Into<ReqSpec>) -> DbResult<RepSpec> {
        assert_eq!(self.nextid, self.recvid.unwrap_or_default());
        let reqid = self.post_request(req)?;
        let reply = self.wait_reply()?;
        // This is implied by the earlier `assert_eq`:
        assert_eq!(reqid, reply.reqid);
        Ok(reply.repspec)
    }

    /// Post a request to the database
    pub fn post_request(&mut self, req: impl Into<ReqSpec>) -> DbResult<Id<Request>> {
        let id = self.nextid.alloc();

        self.to_db
            .send(Request::new(id, req.into()))
            .map_err(|e| self.check_join_error(e))?;

        Ok(id)
    }

    /// Poll for a reply from the database without blocking
    pub fn poll_reply(&mut self) -> DbResult<Option<Reply>> {
        use std::sync::mpsc::{RecvError, TryRecvError::*};

        self.from_db
            .try_recv()
            .map(|r| Some(self.track_reply(r)))
            .or_else(|e| match e {
                Empty => Ok(None),
                Disconnected => Err(self.check_join_error(RecvError)),
            })
    }

    /// Block until a reply is received
    pub fn wait_reply(&mut self) -> DbResult<Reply> {
        let rep = self.from_db.recv().map_err(|e| self.check_join_error(e))?;

        Ok(self.track_reply(rep))
    }

    fn track_reply(&mut self, reply: Reply) -> Reply {
        self.recvid = Some(reply.reqid);
        reply
    }

    fn check_join_error<E>(&mut self, initial: E) -> DbError
    where
        DbError: From<E>,
    {
        use DbError::{Join, Prior};

        self.jh
            .take()
            .ok_or(Prior) // no handle; prior error killed it
            .and_then(|jh| jh.join().map_err(Join))
            .err()
            .unwrap_or_else(|| DbError::from(initial)) // if no more specific error
    }
}
