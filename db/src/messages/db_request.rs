//! DbRequest request type
use derive_more::{From, TryInto, TryIntoError};
use derive_new::new;
use exocortex_redborm::Id;

use crate::entities::{CardSetSynopsisV0, CardV0};
use crate::messages::{
    CardCreate, CardModify, CardModifyG, DbIsEmpty, LogScan, LogScanItems, Queried, Query, Request,
};

/// The top-level request sent by applications to the DB
#[derive(Debug, From, TryInto, new)]
pub enum DbRequest {
    #[allow(missing_docs)]
    #[from(Query, DbIsEmpty, LogScan)]
    Query(Query),
    #[allow(missing_docs)]
    #[from(CardModify)]
    Modify(CardModify),
}

impl Request for DbRequest {
    type Reply = DbReply;
}

impl From<CardCreate> for DbRequest {
    fn from(value: CardCreate) -> Self {
        use CardModifyG::Create;

        DbRequest::Modify(Create(value))
    }
}

impl From<CardSetSynopsisV0> for DbRequest {
    fn from(value: CardSetSynopsisV0) -> Self {
        use CardModifyG::SetSynopsis;

        DbRequest::Modify(SetSynopsis(value))
    }
}

/// The top-level reply sent from the DB to applications
#[derive(Debug, From, TryInto, new)]
pub enum DbReply {
    #[allow(missing_docs)]
    Queried(Queried),
    #[allow(missing_docs)]
    Modified(Id<CardV0>),
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

def_try_into_transitive!(DbReply => Queried => bool);
def_try_into_transitive!(DbReply => Queried => LogScanItems);
