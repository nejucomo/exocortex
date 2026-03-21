use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread::JoinHandle;

use derive_more::From;

use crate::interface::Interface;

pub(crate) fn spawn<F, Req, Rep, E>(f: F) -> (JoinHandle<Result<(), E>>, Interface<Req, Rep>)
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

    let jh = std::thread::spawn(|| child_loop(f, to_from_parent));

    (jh, to_from_child)
}

fn child_loop<F, Req, Rep, Error>(mut f: F, tfp: Interface<Rep, Req>) -> Result<(), Error>
where
    F: FnMut(Req) -> Result<Rep, Error> + Send + 'static,
{
    // It's ok to drop `RecvError` which indicates the parent hung up:
    while let Ok(request) = tfp.from.recv() {
        let rep = f(request)?;
        if tfp.to.send(rep).is_err() {
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
