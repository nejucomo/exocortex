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
