//! The `exocortex` database
#![deny(missing_docs, unsafe_code)]

mod db;
mod error;
mod handler;
mod id;
mod storeload;
mod tables;
mod timestamp;

pub mod entities;
pub mod messages;

pub use self::db::{Database, DatabaseThreadService};
pub use self::error::{DbError, Result};
pub use self::id::Id;
pub use self::timestamp::Timestamp;
