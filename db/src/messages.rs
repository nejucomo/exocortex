//! Message types sent to/from the DB
use derive_more::{From, TryInto, TryIntoError};
use derive_new::new;

use crate::entities::Card;
use crate::{Id, Timestamped};

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

pub(crate) trait Request {
    type Reply;
}

/// The top-level request sent by applications to the DB
#[derive(Debug, From, TryInto, new)]
pub enum DbRequest {
    #[allow(missing_docs)]
    #[from(Query, DbIsEmpty, LogScan)]
    Query(Query),
    #[allow(missing_docs)]
    #[from(Modify, CardCreate, CardModify)]
    Modify(Modify),
}

def_try_into_transitive!(DbRequest => Query => DbIsEmpty);
def_try_into_transitive!(DbRequest => Query => LogScan);
def_try_into_transitive!(DbRequest => Modify => CardCreate);
def_try_into_transitive!(DbRequest => Modify => CardModify);

impl Request for DbRequest {
    type Reply = DbReply;
}

/// The top-level reply sent from the DB to applications
#[derive(Debug, From, TryInto, new)]
pub enum DbReply {
    #[allow(missing_docs)]
    Queried(Queried),
    #[allow(missing_docs)]
    Modified(Id<Card>),
}

def_try_into_transitive!(DbReply => Queried => bool);
def_try_into_transitive!(DbReply => Queried => ScannedItems);

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

/// A query if the db is empty (ie: newly created)
#[derive(Copy, Clone, Debug)]
pub struct DbIsEmpty;

impl Request for DbIsEmpty {
    type Reply = bool;
}

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
pub type ScanItem = (Id<Timestamped<Modify>>, Timestamped<Modify>);

/// A request to modify cards
#[derive(Debug, From, TryInto)]
pub enum Modify {
    #[allow(missing_docs)]
    #[from(CardCreate)]
    CardCreate(CardCreate),
    #[allow(missing_docs)]
    #[from(CardModify)]
    CardModify(CardModify),
}

impl Request for Modify {
    type Reply = Id<Card>;
}

/// A request to create a new card
#[derive(Copy, Clone, Debug)]
pub struct CardCreate;

/// A request to modify a specific card
#[derive(Debug, From, new)]
pub struct CardModify {
    #[allow(missing_docs)]
    pub card: Id<Card>,
    #[allow(missing_docs)]
    #[new(into)]
    pub modif: CardModification,
}

impl Request for CardModify {
    type Reply = Id<Card>;
}

/// A requested modification of a card
#[derive(Debug, From, TryInto)]
pub enum CardModification {
    #[allow(missing_docs)]
    SetSynopsis(CardSetSynopsis),
}

/// A request to set a card synopsis
#[derive(Debug, From, new)]
pub struct CardSetSynopsis(pub String);
