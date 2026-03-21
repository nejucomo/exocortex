/// The database error type
pub type Error = redb::Error;

/// The database error type [Result](std::result::Result) shorthand
pub type Result<T> = std::result::Result<T, Error>;
