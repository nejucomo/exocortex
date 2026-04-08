use std::path::Path;

use exocortex_handler::PollHandler;
use exocortex_memory::{Provider, Reply, Request};
use exocortex_tserv::ThreadService;
use redb::Database;

use crate::redhandler::MemImpl;
use crate::{RedError, RedResult};

#[derive(derive_more::Debug)]
pub struct RedMem {
    svc: ThreadService<MemImpl, Request>,
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
        let redb = Database::create(dbpath)?;
        log::debug!("Launching database service thread.");
        let svc = ThreadService::launch(MemImpl::from(redb));
        Ok(Self { svc })
    }
}

impl Provider for RedMem {
    type Error = RedError;
}

impl PollHandler<Request> for RedMem {
    type Reply = Reply;
    type PollError = RedError;

    fn post_request(&mut self, request: Request) -> RedResult<()> {
        self.svc.post_request(request)
    }

    fn poll_reply(&mut self) -> RedResult<Option<Self::Reply>> {
        self.svc.poll_reply()
    }

    fn wait_reply(&mut self) -> RedResult<Self::Reply> {
        self.svc.wait_reply()
    }
}
