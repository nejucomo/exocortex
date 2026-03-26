//! Message types sent to/from the DB

mod card_create;
mod card_modify;
mod card_set_synopsis;
mod db_is_empty;
mod db_request;
mod log_scan;
mod query;

pub(crate) trait Request {
    type Reply;
}

pub use self::card_create::CardCreate;
pub use self::card_modify::CardModify;
pub use self::card_set_synopsis::CardSetSynopsis;
pub use self::db_is_empty::DbIsEmpty;
pub use self::db_request::{DbReply, DbRequest};
pub use self::log_scan::{LogScan, ScanItem, ScannedItems};
pub use self::query::{Queried, Query};
