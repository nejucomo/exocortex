use moveslot::{MapInPlace as _, MoveSlot};

use crate::svcinner::SvcInner;

/// Run a request -> reply child thread
///
/// This service does not accept any new request until the child has sent any pending reply. Any panic in the child propagates to the parent.
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
    /// Launch the service, serving requests with the given handler
    pub fn launch<F>(f: F) -> Self
    where
        F: FnMut(Req) -> Result<Rep, Error> + Send + 'static,
    {
        Self(MoveSlot::from(SvcInner::launch(f)))
    }

    /// Attempt to post a request to the service, returning that request undelivered if there's a pending reply
    pub fn post_request(&mut self, request: Req) -> Result<Option<Req>, Error> {
        self.0.mip_out_res(|inner| inner.post_request(request))
    }

    /// Check if there is a reply without blocking
    pub fn poll_reply(&mut self) -> Result<Option<Rep>, Error> {
        self.0.mip_out_res(|inner| inner.poll_reply())
    }

    /// Block until there is a reply
    pub fn wait_reply(&mut self) -> Result<Rep, Error> {
        self.0.mip_out_res(|inner| inner.wait_reply())
    }
}
