//! Message types sent to/from the DB

mod blurb_create;
mod blurb_modify;
mod db_is_empty;
mod db_request;
mod log_scan;
mod query;

pub(crate) trait Request {
    type Reply;
}

pub use self::blurb_create::BlurbCreate;
pub use self::blurb_modify::{BlurbModified, BlurbModify, BlurbModifyG};
pub use self::db_is_empty::DbIsEmpty;
pub use self::db_request::{DbReply, DbRequest};
pub use self::log_scan::{LogScan, LogScanItem, LogScanItems};
pub use self::query::{Queried, Query};
