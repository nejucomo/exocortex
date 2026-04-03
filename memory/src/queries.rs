//! Queries

mod scan;

use derive_more::{From, TryInto};
use derive_new::new;
use exocortex_lid::Id;

use crate::modifications::ThopModify;

pub use self::scan::{Scan, ScanNext, ScanQueried, ScanQuery, ScanRelease, ScanReleased};

/// A query of the db
#[derive(Debug, From, TryInto)]
pub enum Query {
    ThopCount(ThopCount),
    Scan(ScanQuery),
}

#[derive(Copy, Clone, Debug)]
pub struct ThopCount;

#[derive(Debug, From, TryInto)]
pub enum Queried {
    ThopCounted(ThopCounted),
    #[from(ScanQueried, Id<Scan>, ThopModify, ScanReleased)]
    Scanned(ScanQueried),
}

#[derive(Copy, Clone, Debug, From, new)]
pub struct ThopCounted(pub u64);
