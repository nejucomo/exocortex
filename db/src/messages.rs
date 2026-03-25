//! Message types sent to/from the DB

mod card_modify;
mod db_is_empty;
mod db_request;
mod log_scan;
mod modify;
mod query;

pub(crate) trait Request {
    type Reply;
}

pub use self::card_modify::{CardModification, CardModify, CardSetSynopsis};
pub use self::db_is_empty::DbIsEmpty;
pub use self::db_request::{DbReply, DbRequest};
pub use self::log_scan::{LogScan, ScanItem, ScannedItems};
pub use self::modify::{CardCreate, Modify};
pub use self::query::{Queried, Query};
