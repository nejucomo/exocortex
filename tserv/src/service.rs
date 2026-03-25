use moveslot::{MapInPlace as _, MoveSlot};

use crate::svcinner::SvcInner;

/// Run a request -> reply child thread
///
/// This service does not accept any new request until the child has sent any pending reply. Any panic in the child propagates to the parent.
#[derive(Debug)]
pub struct ThreadService<Request, Reply, Error>(MoveSlot<SvcInner<Request, Reply, Error>>)
where
    Request: Send + 'static,
    Reply: Send + 'static,
    Error: std::error::Error + Send + 'static;

impl<Request, Reply, Error> ThreadService<Request, Reply, Error>
where
    Request: Send + 'static,
    Reply: Send + 'static,
    Error: std::error::Error + Send + 'static,
{
    /// Launch the service, serving requests with the given handler
    pub fn launch<F>(f: F) -> Self
    where
        F: FnMut(Request) -> Result<Reply, Error> + Send + 'static,
    {
        Self(MoveSlot::from(SvcInner::launch(f)))
    }

    /// Post a request and block on a reply synchronously
    pub fn request<Req, Rep>(&mut self, request: Req) -> Result<Rep, Error>
    where
        Req: Into<Request> + TryFrom<Request, Error: std::fmt::Debug>,
        Rep: TryFrom<Reply>,
        Error: From<Rep::Error>,
    {
        let opt = self.post_request(request)?;
        assert!(opt.is_none());
        self.wait_reply()
    }

    /// Attempt to post a request to the service, returning that request undelivered if there's a pending reply
    pub fn post_request<R>(&mut self, request: R) -> Result<Option<Request>, Error>
    where
        R: Into<Request>,
    {
        self.0
            .mip_out_res(|inner| inner.post_request(request.into()))
    }

    /// Check if there is a reply without blocking
    pub fn poll_reply<R>(&mut self) -> Result<Option<R>, Error>
    where
        R: TryFrom<Reply>,
        Error: From<R::Error>,
    {
        if let Some(rep) = self.0.mip_out_res(|inner| inner.poll_reply())? {
            let r = R::try_from(rep)?;
            Ok(Some(r))
        } else {
            Ok(None)
        }
    }

    /// Block until there is a reply
    pub fn wait_reply<R>(&mut self) -> Result<R, Error>
    where
        R: TryFrom<Reply>,
        Error: From<R::Error>,
    {
        let rep = self.0.mip_out_res(|inner| inner.wait_reply())?;
        let r = R::try_from(rep)?;
        Ok(r)
    }
}
