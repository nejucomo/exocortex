//! LogScan request type
use crate::{Id, Timestamped};

use crate::messages::{CardModify, Request};

/// Scan the log
#[derive(Copy, Clone, Debug)]
pub struct LogScan;

impl Request for LogScan {
    type Reply = ScannedItems;
}

/// All of the items scanned in the order scanned
///
/// # TODO
///
/// Implement pagination, or in-progress stateful scanning
pub type ScannedItems = Vec<ScanItem>;

/// An individual item scanned
pub type ScanItem = (Id<CardModify>, Timestamped<CardModify>);
