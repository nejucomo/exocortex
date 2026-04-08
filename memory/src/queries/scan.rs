use derive_more::{From, TryInto};
use derive_new::new;
use exocortex_lid::{Id, WithId};

use crate::modifications::ThopModified;

/// A marker type for scan session identifiers
#[derive(Copy, Clone, Debug)]
pub struct Scan;

/// A scan protocol query
#[derive(Copy, Clone, Debug, From, TryInto)]
pub enum ScanQuery {
    /// Start a new scan, returns a scan session [`Id`]
    Start(Scan),
    /// Advance an existing scan to the next item
    Advance(ScanNext),
    /// Release (cancel) a scan session
    Release(ScanRelease),
}

/// Advance an existing scan to the next item
#[derive(Copy, Clone, Debug, From, new)]
pub struct ScanNext(pub Id<Scan>);

/// Release (cancel) a scan session
#[derive(Copy, Clone, Debug, From, new)]
pub struct ScanRelease(pub Id<Scan>);

/// The result of a scan query
#[derive(Debug, From, TryInto)]
pub enum ScanQueried {
    /// A scan was started; contains the new scan session id
    Started(Id<Scan>),
    /// The scan advanced; contains the next item
    Advanced(WithId<ThopModified>),
    /// The scan is complete or was released
    Released(ScanReleased),
}

/// Indicates a scan session has ended
#[derive(Copy, Clone, Debug)]
pub struct ScanReleased;
