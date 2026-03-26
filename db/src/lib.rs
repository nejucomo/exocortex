//! The `exocortex` database
#![deny(missing_docs, unsafe_code)]

mod db;
mod dbio;
mod error;
mod handler;
mod id;
mod tables;
mod timestamp;
mod timestamped;

pub mod entities;
pub mod messages;

pub use self::db::{Database, DatabaseThreadService};
pub use self::error::{DbError, Result};
pub use self::id::Id;
pub use self::timestamp::Timestamp;
pub use self::timestamped::Timestamped;
