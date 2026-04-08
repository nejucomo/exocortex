use derive_more::TryIntoError;
use exocortex_handler::{PollHandler, SyncHandler};

use crate::{Reply, ReplyInfo, Request};

/// A memory provider: the primary interface for storing and retrieving thops
pub trait Provider: PollHandler<Request, Reply = Reply, PollError = Self::Error> {
    /// The error type returned by all provider operations
    type Error: std::error::Error + From<TryIntoError<ReplyInfo>>;

    /// Send a request and synchronously wait for the matching reply, returning the extracted reply value
    fn sync_request<Req, Rep>(&mut self, request: Req) -> Result<Rep, Self::Error>
    where
        Self: Sized,
        Req: Into<Request>,
        Rep: TryFrom<ReplyInfo>,
        Self::Error: From<Rep::Error>,
    {
        let req = request.into();
        let reply = <Self as SyncHandler<Request>>::handle(self, req)?;
        // Note: we do not assert reply.request == req here because Request does
        // not implement PartialEq, and the single-request-at-a-time protocol of
        // PollHandler guarantees replies correspond to requests.
        let rep = Rep::try_from(reply.reply_info)?;
        Ok(rep)
    }
}
