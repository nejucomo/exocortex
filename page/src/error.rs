use derive_more::From;
use derive_new::new;
use thiserror::Error;

#[derive(Debug, Error, new)]
#[error("Error with page: {path:?}")]
pub struct PageError {
    #[new(into)]
    pub path: String,
    #[new(into)]
    pub reason: PageErrorReason,
}

aliri_braid::from_infallible!(PageError);

#[derive(Debug, Error, From)]
pub enum PageErrorReason {
    #[error(transparent)]
    InvalidPath(InvalidPath),
    #[error("no page found")]
    Nonexistent,
    #[error("the page contained malformed utf8")]
    MalformedUtf8,
}

#[derive(Debug, Error)]
pub enum InvalidPath {
    #[error("empty segment or multiple adjacent separators")]
    EmptySegment,
    #[error("disallowed leading/trailing whitespace in a segment")]
    ForbiddenWhitespace,
}
