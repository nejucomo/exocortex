//! The `exocortex` <u>req</u>uest/<u>rep</u>ly nano-framework
#![deny(missing_docs, unsafe_code)]

/// A given [Request] is simply associated with a given [Reply](Request::Reply)
pub trait Request {
    /// The reply type for this type of request
    type Reply;
}

/// A base trait for [Queryable] / [Updatable] to define how those handlers may [Err]
pub trait RequestError {
    /// Errors [Self] may produce when queried or updated:
    type Error: std::error::Error;
}

/// Types which can be queried
pub trait Queryable<Q: Request>: RequestError {
    /// Query `self` with a `request`
    fn query(&self, request: Q) -> Result<Q::Reply, Self::Error>;
}

/// Types which can be updated with a request to produce a response
pub trait Updatable<U: Request>: RequestError {
    /// Update `self` given `request`
    fn update(&mut self, request: U) -> Result<U::Reply, Self::Error>;
}
