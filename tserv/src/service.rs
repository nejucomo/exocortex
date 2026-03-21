use moveslot::{MapInPlace as _, MoveSlot};

use crate::ReqRepRes;
use crate::svcinner::SvcInner;

#[derive(Debug)]
pub struct ThreadService<Req, Rep, Error>(MoveSlot<SvcInner<Req, Rep, Error>>)
where
    Req: Send + 'static,
    Rep: Send + 'static,
    Error: std::error::Error + Send + 'static;

impl<Req, Rep, Error> ThreadService<Req, Rep, Error>
where
    Req: Send + 'static,
    Rep: Send + 'static,
    Error: std::error::Error + Send + 'static,
{
    pub fn launch<F>(f: F) -> Self
    where
        F: FnMut(Req) -> Result<Rep, Error> + Send + 'static,
    {
        Self(MoveSlot::from(SvcInner::launch(f)))
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
