//! LogScan request type
use exocortex_redborm::Id;

use crate::Timestamped;
use crate::entities::BlurbModificationV0;
use crate::messages::{BlurbModified, Request};

/// Scan the log
#[derive(Copy, Clone, Debug)]
pub struct LogScan;

impl Request for LogScan {
    type Reply = LogScanItems;
}

/// All of the items scanned in the order scanned
///
/// # TODO
///
/// Implement pagination, or in-progress stateful scanning
pub type LogScanItems = Vec<LogScanItem>;

/// An individual item scanned
pub type LogScanItem = (Id<BlurbModificationV0>, Timestamped<BlurbModified>);
