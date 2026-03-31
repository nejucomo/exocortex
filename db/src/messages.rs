//! Message types sent to/from the DB

mod db_is_empty;
mod db_request;
mod log_scan;
mod query;
mod thop_create;
mod thop_modify;

pub(crate) trait Request {
    type Reply;
}

pub use self::db_is_empty::DbIsEmpty;
pub use self::db_request::{DbReply, DbRequest};
pub use self::log_scan::{LogScan, LogScanItem, LogScanItems};
pub use self::query::{Queried, Query};
pub use self::thop_create::ThopCreate;
pub use self::thop_modify::{ThopModified, ThopModify, ThopModifyG};
