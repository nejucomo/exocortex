#![deny(unsafe_code)]

pub trait PollHandler<Request> {
    type Reply;
    type PollError: std::error::Error;

    fn post_request(&mut self, request: Request) -> Result<(), Self::PollError>;

    fn poll_reply(&mut self) -> Result<Option<Self::Reply>, Self::PollError>;

    fn wait_reply(&mut self) -> Result<Self::Reply, Self::PollError>;

    fn post_subrequest<R: Into<Request>>(&mut self, request: R) -> Result<(), Self::PollError> {
        self.post_request(request.into())
    }

    fn poll_subreply<R>(&mut self) -> Result<Option<R>, Self::PollError>
    where
        R: TryFrom<Self::Reply>,
        Self::PollError: From<R::Error>,
    {
        self.poll_reply().and_then(|optrep| {
            optrep
                .map(|rep| rep.try_into().map_err(Self::PollError::from))
                .transpose()
        })
    }
}

pub trait SyncHandler<Request> {
    type Reply;
    type SyncError: std::error::Error;

    fn handle(&mut self, request: Request) -> Result<Self::Reply, Self::SyncError>;

    fn handle_subrequest<Req, Rep>(&mut self, request: Req) -> Result<Rep, Self::SyncError>
    where
        Req: Into<Request>,
        Rep: TryFrom<Self::Reply>,
        Self::SyncError: From<Rep::Error>,
    {
        let reply = self.handle(request.into())?;
        let rep = Rep::try_from(reply)?;
        Ok(rep)
    }
}

impl<B, R> SyncHandler<R> for B
where
    B: PollHandler<R>,
{
    type Reply = B::Reply;
    type SyncError = B::PollError;

    fn handle(&mut self, request: R) -> Result<Self::Reply, Self::SyncError> {
        self.post_request(request)?;
        self.wait_reply()
    }
}

pub trait SendSyncHandler<R: Send + 'static>:
    Send + SyncHandler<R, Reply: Send + 'static, SyncError: Send + 'static> + 'static
{
}

impl<B, R> SendSyncHandler<R> for B
where
    R: Send + 'static,
    B: Send + SyncHandler<R, Reply: Send + 'static, SyncError: Send + 'static> + 'static,
{
}
