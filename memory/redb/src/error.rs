use derive_more::From;
use thiserror::Error;

/// A [RedError] [Result]
pub type RedResult<T> = Result<T, RedError>;

/// An error in the db layer
#[derive(Debug, Error, From)]
pub enum RedError {
    /// An underlying [exocortex_redborm::OrmError]
    #[error(transparent)]
    #[from(forward)]
    Orm(exocortex_redborm::OrmError),
}
