use derive_more::{From, TryIntoError};
use thiserror::Error;

use crate::messages::DbReply;

/// A [DbError] [Result]
pub type DbResult<T> = Result<T, DbError>;

/// An error in the db layer
#[derive(Debug, Error, From)]
pub enum DbError {
    /// An underlying [exocortex_redborm::OrmError]
    #[error(transparent)]
    #[from(exocortex_redborm::OrmError, std::io::Error, redb::DatabaseError)]
    Orm(exocortex_redborm::OrmError),

    /// An incoherent reply from the db
    #[error(transparent)]
    #[from(TryIntoError<DbReply>)]
    TryFromReply(TryIntoError<DbReply>),
}

impl From<std::convert::Infallible> for DbError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!("<DbError as From<Infallible>::from(...)");
    }
}
