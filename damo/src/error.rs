//! damo error types

use thiserror::Error;

use crate::Id;

/// A damo result
pub type DamoResult<T> = Result<T, DamoError>;

/// An unknown [Id] was passed to the API
#[derive(Debug, Error)]
pub enum DamoError {
    #[error("Unknown ID {:?}", .0)]
    UnknownId(Id),

    #[error("db error: {}", .0)]
    Redb(#[from] redb::DatabaseError),
}
