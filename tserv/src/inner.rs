use std::any::Any;
use std::sync::mpsc::{TryRecvError, TrySendError};
use std::thread::JoinHandle;

use derive_new::new;

use crate::{Interface, ReqRepError, ReqRepRes};

#[derive(Debug, new)]
#[new(visbility = "pub(super)")]
pub(super) struct Inner<Req, Rep, Error>
where
    Req: Send + 'static,
    Rep: Send + 'static,
    Error: std::error::Error + Send + 'static,
{
    jh: JoinHandle<Result<(), Error>>,
    iface: Interface<Req, Rep>,
}

impl<Req, Rep, Error> Inner<Req, Rep, Error>
where
    Req: Send + 'static,
    Rep: Send + 'static,
    Error: std::error::Error + Send + 'static,
{
    pub(super) fn post_request(self, request: Req) -> ReqRepRes<(Self, Option<Req>), Error> {
        use TrySendError::*;

        match self.iface.to.try_send(request) {
            Ok(()) => Ok((self, None)),
            Err(Full(req)) => Ok((self, Some(req))),
            Err(Disconnected(_)) => incorporate_join_error(self.jh.join()),
        }
    }

    pub fn poll_reply(self) -> ReqRepRes<(Self, Option<Rep>), Error> {
        use TryRecvError::*;

        match self.iface.from.try_recv() {
            Ok(rep) => Ok((self, Some(rep))),
            Err(Empty) => Ok((self, None)),
            Err(Disconnected) => incorporate_join_error(self.jh.join()),
        }
    }

    pub fn wait_reply(self) -> ReqRepRes<(Self, Rep), Error> {
        match self.iface.from.recv() {
            Ok(rep) => Ok((self, rep)),
            Err(_) => incorporate_join_error(self.jh.join()),
        }
    }
}

fn incorporate_join_error<T, Error>(
    joinres: Result<Result<(), Error>, Box<dyn Any + Send + 'static>>,
) -> ReqRepRes<T, Error> {
    let appres = joinres?;
    let () = appres.map_err(ReqRepError::Custom)?;
    panic!("Assertion Failed: send failed but child exited ok");
}
