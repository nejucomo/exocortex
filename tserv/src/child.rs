use std::sync::mpsc::{self, Receiver, SyncSender};

use derive_more::From;

use crate::interface::Interface;
use crate::svcinner::SvcInner;

use InnerReply::{AppReply, Started};

pub(crate) enum InnerReply<Rep> {
    Started,
    AppReply(Rep),
}

impl<Rep> InnerReply<Rep> {
    pub(crate) fn unwrap(self) -> Rep {
        match self {
            Started => panic!("internal tserv synchronization invariant failure"),
            AppReply(r) => r,
        }
    }
}

pub(crate) fn spawn<F, Req, Rep, E>(f: F) -> SvcInner<Req, Rep, E>
where
    F: FnMut(Req) -> Result<Rep, E> + Send + 'static,
    Req: Send + 'static,
    Rep: Send + 'static,
    E: std::error::Error + Send + 'static,
{
    let request = Channel::alloc(0);
    let reply = Channel::alloc(1);

    let to_from_child = Interface::new(request.sender, reply.receiver);
    let to_from_parent = Interface::new(reply.sender, request.receiver);

    log::trace!("db thread spawn...");
    let jh = std::thread::spawn(|| {
        log::trace!("db child sending Started...");
        to_from_parent.to.send(Started).unwrap();
        child_loop(f, to_from_parent)
    });

    // Block until child thread signals ready:
    log::trace!("db parent blocking on Started...");
    assert!(matches!(to_from_child.from.recv().unwrap(), Started));
    log::debug!("db service spawn complete");

    SvcInner::new(jh, to_from_child)
}

fn child_loop<F, Req, Rep, Error>(
    mut f: F,
    tfp: Interface<InnerReply<Rep>, Req>,
) -> Result<(), Error>
where
    F: FnMut(Req) -> Result<Rep, Error> + Send + 'static,
{
    // It's ok to drop `RecvError` which indicates the parent hung up:
    while let Ok(request) = tfp.from.recv() {
        let rep = f(request)?;
        if tfp.to.send(AppReply(rep)).is_err() {
            // The parent hung up, so we silently swallow the reply.
            break;
        }
    }

    // The parent hung up, or we exited
    Ok(())
}

/// Channel sender/receiver endpoints for message `M`
#[derive(Debug, From)]
struct Channel<M> {
    /// The sender endpoint
    sender: SyncSender<M>,
    /// The receiver endpoint
    receiver: Receiver<M>,
}

impl<M> Channel<M> {
    /// Allocate a new channel
    pub fn alloc(bound: usize) -> Self {
        Self::from(mpsc::sync_channel(bound))
    }
}
