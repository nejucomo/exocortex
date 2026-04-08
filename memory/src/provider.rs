use exocortex_handler::{PollHandler, SyncHandler};

use crate::{ReplyInfo, Request};

pub trait Provider: PollHandler<Request, PollError = Self::Error> {
    type Error: std::error::Error;

    fn sync_request<Req, Rep>(&mut self, request: Req) -> Result<Rep, Self::Error>
    where
        Req: Into<Request>,
        Rep: TryFrom<ReplyInfo>,
        Self::Error: From<Rep::Error>,
    {
        let req = request.into();
        let rsave = req.clone();
        let reply = <Self as SyncHandler<Request>>::handle(self, req)?;
        assert_eq!(reply.request, rsave);
        let rep = Rep::try_from(reply.reply_info)?;
        Ok(rep)
    }
}
