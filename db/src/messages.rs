//! Message types sent to/from the DB thread
use std::sync::Arc;

use derive_more::From;
use derive_new::new;

use crate::Id;
use crate::entities::Card;

pub(crate) trait Request {
    type Reply;
}

/// The top-level request sent by applications to the DB
#[derive(Debug, From, new)]
pub enum DbRequest {
    #[allow(missing_docs)]
    #[from(Query, DbIsEmpty, CardScan)]
    Query(Query),
    #[allow(missing_docs)]
    #[from(Modify, CardCreate, CardModify)]
    Modify(Modify),
}

/// The top-level reply sent from the DB to applications
#[derive(Debug, From, new)]
pub enum DbReply {
    #[allow(missing_docs)]
    #[from(Queried, CardScanned)]
    Queried(Queried),
    #[allow(missing_docs)]
    Modified(CardUpdated),
}

impl Request for DbRequest {
    type Reply = DbReply;
}

/// A query of the db
#[derive(Debug, From)]
pub enum Query {
    #[allow(missing_docs)]
    DbIsEmpty(DbIsEmpty),
    #[allow(missing_docs)]
    LogScan(LogScan),
}

/// Reply specifics
#[derive(Debug, From)]
pub enum Queried {
    #[allow(missing_docs)]
    DbWasEmpty(bool),
    #[allow(missing_docs)]
    LogScanned(Vec<Modify>),
}

impl Request for Query {
    type Reply = Queried;
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
    type Reply = Vec<Modify>;
}

/// A request to modify cards
#[derive(Debug, From)]
pub enum Modify {
    #[allow(missing_docs)]
    #[from(CardCreate)]
    CardCreate(CardCreate),
    #[allow(missing_docs)]
    #[from(CardModify)]
    CardModify(CardModify),
}

impl Request for Modify {
    type Reply = ();
}

/// A request to create a new card
#[derive(Copy, Clone, Debug)]
pub struct CardCreate;

impl Request for CardCreate {
    type Reply = Id<Card>;
}

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
    type Reply = ();
}

/// A requested modification of a card
#[derive(Debug, From)]
pub enum CardModification {
    #[allow(missing_docs)]
    SetSynopsis(CardSetSynopsis),
}

impl Request for CardModification {
    type Reply = ();
}

/// A request to set a card synopsis
#[derive(Debug, From, new)]
pub struct CardSetSynopsis(pub String);

impl Request for CardSetSynopsis {
    type Reply = ();
}
