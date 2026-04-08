use derive_more::From;
use thiserror::Error;

use exocortex_memory::ReplyInfo;

/// A [RamError] [Result]
pub type RamResult<T> = Result<T, RamError>;

/// An error from the in-memory provider
#[derive(Debug, Error, From)]
pub enum RamError {
    /// The reply type did not match the expected type for the request
    #[error("unexpected reply type")]
    UnexpectedReply(derive_more::TryIntoError<ReplyInfo>),
}
