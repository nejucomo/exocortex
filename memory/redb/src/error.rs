use thiserror::Error;

use exocortex_memory::ReplyInfo;

/// A [RedError] [Result]
pub type RedResult<T> = Result<T, RedError>;

/// An error in the db layer
#[derive(Debug, Error)]
pub enum RedError {
    /// An underlying [exocortex_redborm::OrmError]
    #[error(transparent)]
    Orm(#[from] exocortex_redborm::OrmError),

    /// The reply type did not match the expected type for the request
    #[error("unexpected reply type")]
    UnexpectedReply(#[from] derive_more::TryIntoError<ReplyInfo>),
}

impl From<redb::CommitError> for RedError {
    fn from(e: redb::CommitError) -> Self {
        RedError::Orm(e.into())
    }
}

impl From<redb::DatabaseError> for RedError {
    fn from(e: redb::DatabaseError) -> Self {
        RedError::Orm(e.into())
    }
}

impl From<redb::StorageError> for RedError {
    fn from(e: redb::StorageError) -> Self {
        RedError::Orm(e.into())
    }
}

impl From<redb::TableError> for RedError {
    fn from(e: redb::TableError) -> Self {
        RedError::Orm(e.into())
    }
}

impl From<redb::TransactionError> for RedError {
    fn from(e: redb::TransactionError) -> Self {
        RedError::Orm(e.into())
    }
}

impl From<std::io::Error> for RedError {
    fn from(e: std::io::Error) -> Self {
        RedError::Orm(e.into())
    }
}
