//! Message types sent to/from the DB

mod card_create;
mod card_modify;
mod db_is_empty;
mod db_request;
mod log_scan;
mod query;

pub(crate) trait Request {
    type Reply;
}

pub use self::card_create::CardCreate;
pub use self::card_modify::{CardModified, CardModify, CardModifyG};
pub use self::db_is_empty::DbIsEmpty;
pub use self::db_request::{DbReply, DbRequest};
pub use self::log_scan::{LogScan, LogScanItem, LogScanItems};
pub use self::query::{Queried, Query};
