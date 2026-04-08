use derive_more::TryIntoError;
use exocortex_handler::{PollHandler, SyncHandler};

use crate::{Reply, ReplyInfo, Request};

pub trait Provider: PollHandler<Request, Reply = Reply, PollError = Self::Error> {
    type Error: std::error::Error + From<TryIntoError<ReplyInfo>>;

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
