//! The `exocortex` database
#![deny(missing_docs, unsafe_code)]

mod db;
mod error;
mod handler;
mod timestamp;
mod timestamped;

pub mod entities;
pub mod messages;

pub use self::db::{Database, DatabaseThreadService};
pub use self::error::{DbError, DbResult};
pub use self::timestamp::Timestamp;
pub use self::timestamped::Timestamped;
