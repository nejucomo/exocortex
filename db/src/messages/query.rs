//! Query request type
use derive_more::{From, TryInto};

use crate::messages::{DbIsEmpty, LogScan, Request, ScannedItems};

/// A query of the db
#[derive(Debug, From, TryInto)]
pub enum Query {
    #[allow(missing_docs)]
    DbIsEmpty(DbIsEmpty),
    #[allow(missing_docs)]
    LogScan(LogScan),
}

impl Request for Query {
    type Reply = Queried;
}

/// Reply specifics
#[derive(Debug, From, TryInto)]
pub enum Queried {
    #[allow(missing_docs)]
    DbWasEmpty(bool),
    #[allow(missing_docs)]
    LogScanned(ScannedItems),
}
