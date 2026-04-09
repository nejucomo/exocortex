use derive_more::From;
use thiserror::Error;

use exocortex_memory::ReplyInfo;

/// A [RedError] [Result]
pub type RedResult<T> = Result<T, RedError>;

/// An error in the db layer
#[derive(Debug, Error, From)]
pub enum RedError {
    /// An underlying [exocortex_redborm::OrmError]
    #[error(transparent)]
    #[from(
        exocortex_redborm::OrmError,
        redb::CommitError,
        redb::DatabaseError,
        redb::StorageError,
        redb::TableError,
        redb::TransactionError,
        std::io::Error
    )]
    Orm(exocortex_redborm::OrmError),

    /// The reply type did not match the expected type for the request
    #[error("unexpected reply type")]
    UnexpectedReply(derive_more::TryIntoError<ReplyInfo>),
}

impl From<derive_more::TryIntoError<ReplyInfo>> for RedError {
    fn from(e: derive_more::TryIntoError<ReplyInfo>) -> Self {
        RedError::UnexpectedReply(e)
    }
}
