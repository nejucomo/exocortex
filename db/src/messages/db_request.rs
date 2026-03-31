//! DbRequest request type
use derive_more::{From, TryInto, TryIntoError};
use derive_new::new;

use crate::ThopId;
use crate::entities::ThopSetSynopsisV0;
use crate::messages::{
    DbIsEmpty, LogScan, LogScanItems, Queried, Query, Request, ThopCreate, ThopModify, ThopModifyG,
};

/// The top-level request sent by applications to the DB
#[derive(Debug, From, TryInto, new)]
pub enum DbRequest {
    #[allow(missing_docs)]
    #[from(Query, DbIsEmpty, LogScan)]
    Query(Query),
    #[allow(missing_docs)]
    #[from(ThopModify, ThopCreate)]
    Modify(ThopModify),
}

impl Request for DbRequest {
    type Reply = DbReply;
}

impl From<ThopSetSynopsisV0> for DbRequest {
    fn from(value: ThopSetSynopsisV0) -> Self {
        use ThopModifyG::SetSynopsis;

        DbRequest::Modify(SetSynopsis(value))
    }
}

/// The top-level reply sent from the DB to applications
#[derive(Debug, From, TryInto, new)]
pub enum DbReply {
    #[allow(missing_docs)]
    Queried(Queried),
    #[allow(missing_docs)]
    Modified(ThopId),
}

macro_rules! def_try_into_transitive {
    ( $a:ty => $b:ty => $c:ty ) => {
        impl TryFrom<$a> for $c {
            type Error = TryIntoError<$a>;

            fn try_from(v: $a) -> Result<Self, Self::Error> {
                v.try_into().and_then(|v2: $b| {
                    v2.try_into().map_err(|e: TryIntoError<$b>| {
                        TryIntoError::new(e.input.into(), "FIXME", "FIXME")
                    })
                })
            }
        }
    };
}

def_try_into_transitive!(DbRequest => Query => DbIsEmpty);
def_try_into_transitive!(DbRequest => Query => LogScan);
def_try_into_transitive!(DbRequest => ThopModify => ThopCreate);
def_try_into_transitive!(DbRequest => ThopModify => ThopSetSynopsisV0);

def_try_into_transitive!(DbReply => Queried => bool);
def_try_into_transitive!(DbReply => Queried => LogScanItems);
