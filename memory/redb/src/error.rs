use derive_more::From;
use thiserror::Error;

/// A [RedError] [Result]
pub type RedResult<T> = Result<T, RedError>;

/// An error in the db layer
#[derive(Debug, Error, From)]
pub enum RedError {
    /// An underlying [exocortex_redborm::OrmError]
    #[error(transparent)]
    #[from(exocortex_redborm::OrmError, std::io::Error, redb::DatabaseError)]
    Orm(exocortex_redborm::OrmError),
    // /// An incoherent reply from the db
    // #[error(transparent)]
    // #[from(TryIntoError<RedReply>)]
    // TryFromReply(TryIntoError<RedReply>),
}

// impl From<std::convert::Infallible> for RedError {
//     fn from(_: std::convert::Infallible) -> Self {
//         unreachable!("<RedError as From<Infallible>::from(...)");
//     }
// }
