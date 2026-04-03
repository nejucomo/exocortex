use derive_more::{From, TryInto};
use exocortex_lid::Id;

use crate::def_transitive_conversion::def_transitive_conversion;
use crate::modifications::ThopModify;
use crate::queries::{Queried, Scan, ScanQueried, ScanReleased, ThopCounted};
use crate::{Request, Thop};

pub struct Reply {
    pub request: Request,
    pub reply_info: ReplyInfo,
}

/// The top-level reply sent from the DB to applications
#[derive(Debug, From, TryInto)]
pub enum ReplyInfo {
    #[from(Queried, ScanQueried)]
    Queried(Queried),
    Modified(Id<Thop>),
}

def_transitive_conversion!(TryInto: ReplyInfo => Queried => ThopCounted);
def_transitive_conversion!(TryInto: ReplyInfo => Queried => ScanQueried);
def_transitive_conversion!(TryInto: ReplyInfo => ScanQueried => Id<Scan>);
def_transitive_conversion!(TryInto: ReplyInfo => ScanQueried => ThopModify);
def_transitive_conversion!(TryInto: ReplyInfo => ScanQueried => ScanReleased);
