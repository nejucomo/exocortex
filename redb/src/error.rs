//! Error types

use std::any::Any;
use std::sync::mpsc;

use derive_more::From;
use thiserror::Error;

use crate::Id;
use crate::messages::{CardScan, CardScanned, RepSpec, Reply, ReqSpec, Request};

/// The result of a db operation
pub type DbResult<T> = Result<T, DbError>;

/// The type of errors which may be unhandled by the db thread
#[derive(Debug, Error, From)]
pub enum DbError {
    /// An error from the underlying database
    #[error(transparent)]
    #[from(
        redb::Error,
        redb::CommitError,
        redb::StorageError,
        redb::TableError,
        redb::TransactionError
    )]
    Redb(redb::Error),

    /// A [SendRequestError]
    #[error(transparent)]
    #[from(mpsc::SendError<Request>, mpsc::SendError<CardScan>)]
    SendRequest(SendRequestError),

    /// A [SendReplyError]
    #[error(transparent)]
    #[from(mpsc::SendError<Reply>, mpsc::SendError<CardScanned>)]
    SendReply(SendReplyError),

    /// the app exited before handling a reply
    #[error("db thread exited without sending an anticipated reply: {:#?}", .0)]
    #[from(mpsc::RecvError)]
    RecvError(mpsc::RecvError),

    /// The db thread panicked
    #[error("db thread panicked: {:#?}", .0)]
    #[from(Box<dyn Any + Send + 'static>)]
    Join(Box<dyn Any + Send + 'static>),

    /// A prior error was unhandled, and there's no running db thread
    #[error("db thread already previously failed")]
    Prior,
}

/// The type of errors which may be unhandled by the db thread
#[derive(Debug, Error)]
#[error("db thread died before request {:?}: {:#?}", .0, .1)]
pub struct SendRequestError(Option<Id<Request>>, ReqSpec);

impl From<mpsc::SendError<Request>> for SendRequestError {
    fn from(e: mpsc::SendError<Request>) -> Self {
        Self(Some(e.0.id), e.0.reqspec)
    }
}

impl<T> From<mpsc::SendError<T>> for SendRequestError
where
    ReqSpec: From<T>,
{
    fn from(e: mpsc::SendError<T>) -> Self {
        Self(None, ReqSpec::from(e.0))
    }
}

/// An error indicating the app exited before handling a reply
#[derive(Debug, Error)]
#[error("app failed to process db reply {:?}: {:#?}", .0, .1)]
pub struct SendReplyError(Option<Id<Request>>, RepSpec);

impl From<mpsc::SendError<Reply>> for SendReplyError {
    fn from(e: mpsc::SendError<Reply>) -> Self {
        Self(Some(e.0.reqid), e.0.repspec)
    }
}

impl<T> From<mpsc::SendError<T>> for SendReplyError
where
    RepSpec: From<T>,
{
    fn from(e: mpsc::SendError<T>) -> Self {
        SendReplyError(None, RepSpec::from(e.0))
    }
}
