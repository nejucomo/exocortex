use std::any::Any;
use std::sync::mpsc;

use derive_more::From;
use thiserror::Error;

use crate::messages::{Reply, Request};

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

    /// the db thread exited before handling a request
    #[error("app failed to process db request: {:#?}", .0.0)]
    #[from(mpsc::SendError<Request>)]
    SendRequest(mpsc::SendError<Request>),

    /// the app exited before handling a reply
    #[error("db exited without sending an anticipated reply: {:#?}", .0)]
    #[from(mpsc::RecvError)]
    RecvError(mpsc::RecvError),

    /// An error indicating the app exited before handling a reply
    #[error("app failed to process db reply: {:#?}", .0.0)]
    #[from(mpsc::SendError<Reply>)]
    SendReply(mpsc::SendError<Reply>),

    /// The db thread panicked
    #[error("db thread panicked: {:#?}", .0)]
    #[from(Box<dyn Any + Send + 'static>)]
    Join(Box<dyn Any + Send + 'static>),

    /// A prior error was unhandled, and there's no running db thread
    #[error("db thread already previously failed")]
    Prior,
}
