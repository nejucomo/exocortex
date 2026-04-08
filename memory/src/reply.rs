use derive_more::{From, TryInto};
use exocortex_lid::{Id, WithId};

use crate::Request;
use crate::def_transitive_conversion::def_transitive_conversion;
use crate::modifications::ThopModified;
use crate::queries::{Queried, Scan, ScanQueried, ScanReleased, ThopCounted};

/// A reply from a [`Provider`](crate::Provider), paired with the originating request
#[derive(Debug)]
pub struct Reply {
    /// The request that produced this reply
    pub request: Request,
    /// The reply payload
    pub reply_info: ReplyInfo,
}

/// The top-level reply sent from the DB to applications
#[derive(Debug, From, TryInto)]
pub enum ReplyInfo {
    /// A query was answered
    #[from(Queried, ScanQueried)]
    Queried(Queried),
    /// A thop was modified; the value includes the modification id and details
    Modified(WithId<ThopModified>),
}

def_transitive_conversion!(TryInto: ReplyInfo => Queried => ThopCounted);
def_transitive_conversion!(TryInto: ReplyInfo => Queried => ScanQueried);
def_transitive_conversion!(TryInto: ReplyInfo => ScanQueried => Id<Scan>);
def_transitive_conversion!(TryInto: ReplyInfo => ScanQueried => ScanReleased);
