use exocortex_handler::PollHandler;
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
}

impl<Request, Reply, Error> PollHandler<Request> for ThreadService<Request, Reply, Error>
where
    Request: Send + 'static,
    Reply: Send + 'static,
    Error: std::error::Error + Send + 'static,
{
    type Reply = Reply;
    type PollError = Error;

    fn post_request(&mut self, request: Request) -> Result<(), Self::PollError> {
        self.0
            .mip_out_res(|inner| inner.post_request(request).map(|inner| (inner, ())))
    }

    fn poll_reply(&mut self) -> Result<Option<Self::Reply>, Self::PollError> {
        self.0.mip_out_res(|inner| inner.poll_reply())
    }

    fn wait_reply(&mut self) -> Result<Self::Reply, Self::PollError> {
        self.0.mip_out_res(|inner| inner.wait_reply())
    }
}
