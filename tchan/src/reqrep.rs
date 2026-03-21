#![allow(missing_docs)]

mod error;
mod inner;

use moveslot::{MapInPlace as _, MoveSlot};

use crate::{Interface, InterfacePair};

use self::inner::Inner;

pub use self::error::{ReqRepError, ReqRepRes};

#[derive(Debug)]
pub struct ParentInterface<Req, Rep, Error>(MoveSlot<Inner<Req, Rep, Error>>)
where
    Req: Send + 'static,
    Rep: Send + 'static,
    Error: std::error::Error + Send + 'static;

pub fn launch<F, Req, Rep, Error>(f: F) -> ParentInterface<Req, Rep, Error>
where
    F: FnMut(Req) -> Result<Rep, Error> + Send + 'static,
    Req: Send + 'static,
    Rep: Send + 'static,
    Error: std::error::Error + Send + 'static,
{
    ParentInterface::launch(f)
}

impl<Req, Rep, Error> ParentInterface<Req, Rep, Error>
where
    Req: Send + 'static,
    Rep: Send + 'static,
    Error: std::error::Error + Send + 'static,
{
    pub fn launch<F>(f: F) -> Self
    where
        F: FnMut(Req) -> Result<Rep, Error> + Send + 'static,
    {
        // FIXME: Pick a better name, given the rendevous-request design.
        let (to_from_child, to_from_parent) = InterfacePair::alloc(0, 1).into();
        let jh = std::thread::spawn(|| child_loop(f, to_from_parent));
        Self(MoveSlot::from(Inner::new(jh, to_from_child)))
    }

    pub fn post_request(&mut self, request: Req) -> ReqRepRes<Option<Req>, Error> {
        self.0.mip_out_res(|inner| inner.post_request(request))
    }

    pub fn poll_reply(&mut self) -> ReqRepRes<Option<Rep>, Error> {
        self.0.mip_out_res(|inner| inner.poll_reply())
    }

    pub fn wait_reply(&mut self) -> ReqRepRes<Rep, Error> {
        self.0.mip_out_res(|inner| inner.wait_reply())
    }
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
