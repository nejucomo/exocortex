use exocortex_handler::{PollHandler, SendSyncHandler};
use moveslot::{MapInPlace as _, MoveSlot};

use crate::svcinner::SvcInner;

/// Run a request -> reply child thread
///
/// This service does not accept any new request until the child has sent any pending reply. Any panic in the child propagates to the parent.
#[derive(derive_more::Debug)]
pub struct ThreadService<H, R>(MoveSlot<SvcInner<H, R>>)
where
    H: SendSyncHandler<R>,
    R: Send + 'static;

impl<H, R> ThreadService<H, R>
where
    H: SendSyncHandler<R>,
    R: Send + 'static,
{
    /// Launch the service, serving requests with the given handler
    pub fn launch(handler: H) -> Self {
        Self(MoveSlot::from(SvcInner::launch(handler)))
    }
}

impl<H, R> PollHandler<R> for ThreadService<H, R>
where
    H: SendSyncHandler<R>,
    R: Send + 'static,
{
    type Reply = H::Reply;
    type PollError = H::SyncError;

    fn post_request(&mut self, request: R) -> Result<(), Self::PollError> {
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
