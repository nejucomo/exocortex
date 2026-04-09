//! Queries

mod scan;

use derive_more::{From, TryInto};
use derive_new::new;
use exocortex_lid::{Id, WithId};

use crate::modifications::ThopModified;

pub use self::scan::{Scan, ScanNext, ScanQueried, ScanQuery, ScanRelease, ScanReleased};

/// A query of the db
#[derive(Debug, From, TryInto)]
pub enum Query {
    /// Count all thops
    ThopCount(ThopCount),
    /// Run a scan query
    Scan(ScanQuery),
}

/// The result of a query
#[derive(Debug, From, TryInto)]
pub enum Queried {
    /// Result of a [`ThopCount`] query
    ThopCounted(ThopCounted),
    /// Result of a scan query
    #[from(ScanQueried, Id<Scan>, WithId<ThopModified>, ScanReleased)]
    Scanned(ScanQueried),
}

/// A request to count all thops
#[derive(Copy, Clone, Debug)]
pub struct ThopCount;

/// The count of all thops
#[derive(Copy, Clone, Debug, From, new)]
pub struct ThopCounted(pub u64);
