use std::sync::mpsc::{TryRecvError, TrySendError};
use std::thread::JoinHandle;

use derive_new::new;

use crate::child;
use crate::interface::Interface;

#[derive(Debug, new)]
#[new(visbility = "pub(crate)")]
pub(crate) struct SvcInner<Req, Rep, Error>
where
    Req: Send + 'static,
    Rep: Send + 'static,
    Error: std::error::Error + Send + 'static,
{
    jh: JoinHandle<Result<(), Error>>,
    iface: Interface<Req, child::InnerReply<Rep>>,
}

impl<Req, Rep, Error> SvcInner<Req, Rep, Error>
where
    Req: Send + 'static,
    Rep: Send + 'static,
    Error: std::error::Error + Send + 'static,
{
    pub(crate) fn launch<F>(f: F) -> Self
    where
        F: FnMut(Req) -> Result<Rep, Error> + Send + 'static,
    {
        child::spawn(f)
    }

    pub(crate) fn post_request(self, request: Req) -> Result<(Self, Option<Req>), Error> {
        use TrySendError::*;

        match self.iface.to.try_send(request) {
            Ok(()) => Ok((self, None)),
            Err(Full(req)) => Ok((self, Some(req))),
            Err(Disconnected(_)) => self.join_unwind(),
        }
    }

    pub fn poll_reply(self) -> Result<(Self, Option<Rep>), Error> {
        use TryRecvError::*;

        match self.iface.from.try_recv() {
            Ok(rep) => Ok((self, Some(rep.unwrap()))),
            Err(Empty) => Ok((self, None)),
            Err(Disconnected) => self.join_unwind(),
        }
    }

    pub fn wait_reply(self) -> Result<(Self, Rep), Error> {
        match self.iface.from.recv() {
            Ok(rep) => Ok((self, rep.unwrap())),
            Err(_) => self.join_unwind(),
        }
    }

    fn join_unwind<T>(self) -> Result<T, Error> {
        match self.jh.join() {
            Ok(Ok(())) => panic!("expected child error"),
            Ok(Err(e)) => Err(e),
            // Propagate child panics:
            Err(e) => std::panic::resume_unwind(e),
        }
    }
}
