use std::sync::mpsc::{TryRecvError, TrySendError};
use std::thread::JoinHandle;

use derive_new::new;
use exocortex_handler::SendSyncHandler;

use crate::child;
use crate::interface::Interface;

#[derive(derive_more::Debug, new)]
#[new(visbility = "pub(crate)")]
pub(crate) struct SvcInner<H, R>
where
    H: SendSyncHandler<R>,
    R: Send + 'static,
{
    jh: JoinHandle<Result<(), H::SyncError>>,
    iface: Interface<R, child::InnerReply<H::Reply>>,
}

impl<H, R> SvcInner<H, R>
where
    H: SendSyncHandler<R>,
    R: Send + 'static,
{
    pub(crate) fn launch(handler: H) -> Self {
        child::spawn(handler)
    }

    pub(crate) fn post_request(self, request: R) -> Result<Self, H::SyncError> {
        use TrySendError::*;

        match self.iface.to.try_send(request) {
            Ok(()) => Ok(self),
            Err(Full(_)) => panic!("channel full"),
            Err(Disconnected(_)) => self.join_unwind(),
        }
    }

    pub fn poll_reply(self) -> Result<(Self, Option<H::Reply>), H::SyncError> {
        use TryRecvError::*;

        match self.iface.from.try_recv() {
            Ok(rep) => Ok((self, Some(rep.unwrap()))),
            Err(Empty) => Ok((self, None)),
            Err(Disconnected) => self.join_unwind(),
        }
    }

    pub fn wait_reply(self) -> Result<(Self, H::Reply), H::SyncError> {
        match self.iface.from.recv() {
            Ok(rep) => Ok((self, rep.unwrap())),
            Err(_) => self.join_unwind(),
        }
    }

    fn join_unwind<T>(self) -> Result<T, H::SyncError> {
        match self.jh.join() {
            Ok(Ok(())) => panic!("expected child error"),
            Ok(Err(e)) => Err(e),
            // Propagate child panics:
            Err(e) => std::panic::resume_unwind(e),
        }
    }
}
